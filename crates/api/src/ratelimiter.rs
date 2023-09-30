use std::str::FromStr;
use std::sync::Arc;
use std::time::SystemTime;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Request, RequestBuilder, Response, StatusCode, Url};
use tokio::sync::{Mutex, RwLock};
use tokio::time::{sleep, Duration};
use tracing::info;
use translate::HttpError;

pub type Result<T> = std::result::Result<T, HttpError>;

#[derive(Debug)]
pub enum Route {
	Hypixel,
	Mojang,
}

#[derive(Default)]
pub struct Routes {
	hypixel: Arc<Mutex<Ratelimit>>,
	mojang: Arc<Mutex<Ratelimit>>,
}

impl Routes {
	#[must_use]
	pub fn get(&self, route: &Route) -> Arc<Mutex<Ratelimit>> {
		match route {
			Route::Hypixel => Arc::clone(&self.hypixel),
			Route::Mojang => Arc::clone(&self.mojang),
		}
	}
}

pub struct RatelimitInfo {
	pub timeout: std::time::Duration,
	pub limit: i64,
	pub path: String,
	pub global: bool,
}

pub struct Ratelimiter {
	pub client: Client,
	token: HeaderValue,
	routes: Arc<RwLock<Routes>>,
}

impl Default for Ratelimit {
	fn default() -> Self {
		Self {
			limit: i64::MAX,
			remaining: i64::MAX,
			reset: None,
			reset_after: None,
		}
	}
}

impl Ratelimiter {
	/// Creates a new ratelimiter. `client` should contain the
	/// `API-Key` header already.
	#[must_use]
	pub fn new(client: Client, token: HeaderValue) -> Self {
		Self {
			client,
			token,
			routes: Arc::default(),
		}
	}

	#[must_use]
	pub fn routes(&self) -> Arc<RwLock<Routes>> {
		Arc::clone(&self.routes)
	}

	#[inline]
	pub fn get(&self, url: Url) -> RequestBuilder {
		self.client.get(url)
	}

	#[inline]
	pub fn post(&self, url: Url) -> RequestBuilder {
		self.client.post(url)
	}

	/// # Errors
	/// Returns an error if the header is not present, or if the header is invalid.
	#[inline]
	pub async fn perform_hypixel(&self, mut req: RatelimitedRequest) -> Result<Response> {
		let headers = req.req.headers_mut();

		headers.insert("API-Key", self.token.clone());

		self.perform(req, Route::Hypixel).await
	}

	/// # Errors
	/// Returns an error if the header is not present, or if the header is invalid.
	#[inline]
	pub async fn perform_mojang(&self, req: RatelimitedRequest) -> Result<Response> {
		self.perform(req, Route::Mojang).await
	}

	/// Sends a regular request
	///
	/// # Errors
	/// Transparently returns [`reqwest::Error`]
	#[inline]
	pub async fn perform_bare(&self, req: Request) -> reqwest::Result<Response> {
		self.client.execute(req).await
	}

	async fn perform(&self, req: RatelimitedRequest, route: Route) -> Result<Response> {
		let RatelimitedRequest { req } = req;

		loop {
			let bucket = self.routes.write().await.get(&route);

			bucket.lock().await.pre_hook().await;

			// This will not panic, since the request body is never a stream.
			let response = self.client.execute(req.try_clone().unwrap()).await?;
			let redo = bucket.lock().await.post_hook(&response).await;

			if !redo.unwrap_or(true) {
				return Ok(response);
			}
		}
	}
}

fn parse_header<T: FromStr>(headers: &HeaderMap, header: &str) -> Result<Option<T>> {
	let Some(header) = headers.get(header) else {
		return Ok(None);
	};

	let unicode =
		std::str::from_utf8(header.as_bytes()).map_err(|_| HttpError::InvalidHeaderUtf8)?;
	let num = unicode.parse().map_err(|_| HttpError::InvalidHeaderUtf8)?;

	Ok(Some(num))
}

impl Ratelimit {
	pub async fn pre_hook(&mut self) {
		if self.limit() == 0 {
			return;
		}

		let Some(reset) = self.reset else {
			// We're probably in the past.
			self.remaining = self.limit;

			return;
		};

		let Ok(delay) = reset.duration_since(SystemTime::now()) else {
			// if duration is negative (i.e. adequate time has passed since last call to this api)
			if self.remaining() != 0 {
				self.remaining -= 1;
			}

			return;
		};

		if self.remaining() == 0 {
			info!(sleep = delay.as_secs_f64(), "ratelimit reached");
			sleep(delay).await;

			return;
		}

		self.remaining -= 1;
	}

	/// # Errors
	/// Returns an error if the header is not a valid utf-8 string or is in the wrong format.
	pub async fn post_hook(&mut self, response: &Response) -> Result<bool> {
		if let Some(limit) = parse_header(response.headers(), "ratelimit-limit")? {
			self.limit = limit;
		}

		if let Some(remaining) = parse_header(response.headers(), "ratelimit-remaining")? {
			self.remaining = remaining;
		}

		if let Some(reset_after) = parse_header::<f64>(response.headers(), "ratelimit-reset")? {
			self.reset = Some(SystemTime::now() + Duration::from_secs_f64(reset_after));
			self.reset_after = Some(Duration::from_secs_f64(reset_after));
		}

		Ok(if response.status() != StatusCode::TOO_MANY_REQUESTS {
			false
		} else if let Some(retry_after) =
			// If the header does not exist (like the case is with Mojang), just wait 5 seconds.
			parse_header::<f64>(response.headers(), "retry-after").unwrap_or(Some(5.))
		{
			info!(sleep = retry_after, "ratelimit reached");
			sleep(Duration::from_secs_f64(retry_after)).await;

			true
		} else {
			false
		})
	}

	/// The total number of requests that can be made in a period of time.
	#[inline]
	#[must_use]
	pub fn limit(&self) -> i64 {
		self.limit
	}

	/// The number of requests remaining in the period of time.
	#[inline]
	#[must_use]
	pub fn remaining(&self) -> i64 {
		self.remaining
	}

	/// The absolute time in milliseconds when the interval resets.
	#[inline]
	#[must_use]
	pub fn reset(&self) -> Option<SystemTime> {
		self.reset
	}

	/// The total time in milliseconds when the interval resets.
	#[inline]
	#[must_use]
	pub fn reset_after(&self) -> Option<Duration> {
		self.reset_after
	}
}

#[derive(Debug)]
pub struct Ratelimit {
	/// The total number of requests that can be made in a period of time.
	limit: i64,
	/// The number of requests remaining in the period of time.
	remaining: i64,
	/// The absolute time when the interval resets.
	reset: Option<SystemTime>,
	/// The total time when the interval resets.
	reset_after: Option<Duration>,
}

/// Information about a request for the ratelimiter to perform.
///
/// This only contains the basic information needed by the ratelimiter to
/// perform a full cycle of making the request and returning the response.
///
/// Use the [`From`] implementations for making one of these.
#[derive(Debug)]
pub struct RatelimitedRequest {
	req: Request,
}

impl From<Request> for RatelimitedRequest {
	fn from(req: Request) -> Self {
		Self { req }
	}
}
