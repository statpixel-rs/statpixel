use futures::{
	channel::{mpsc, oneshot},
	SinkExt, StreamExt,
};
use http_cache_reqwest::{CacheMode, HttpCache, MokaManager};
use moka::future::CacheBuilder;
use once_cell::sync::Lazy;
use reqwest::{Request, Response};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use std::time::Duration;
use tower::{Service, ServiceExt};

use crate::Error;

#[derive(Debug)]
pub struct HttpClient {
	request_tx: mpsc::Sender<(Request, oneshot::Sender<Result<Response, Error>>)>,
}

#[inline]
async fn service_fn(
	client: ClientWithMiddleware,
	req: Request,
) -> Result<Response, reqwest_middleware::Error> {
	client.execute(req).await
}

impl HttpClient {
	/// [buffer] -> [concurrency req pool] - :{rate limit}: -> client.call()
	pub fn new(
		client: ClientWithMiddleware,
		channel_buffer_size: usize,
		request_buffer_size: usize,
		max_concurrency_number: usize,
		rate_limit_number: u64,
		rate_limit_duration: Duration,
	) -> Self {
		let (tx, rx) = mpsc::channel::<(Request, oneshot::Sender<Result<Response, Error>>)>(
			channel_buffer_size,
		);

		tokio::spawn(async move {
			let client = client.clone();

			let service = tower::ServiceBuilder::new()
				.buffer(request_buffer_size)
				.concurrency_limit(max_concurrency_number)
				.rate_limit(rate_limit_number, rate_limit_duration)
				.service(tower::util::service_fn(move |req| {
					service_fn(client.clone(), req)
				}));

			rx.for_each_concurrent(max_concurrency_number, move |(req, resp_tx)| {
				let mut inner_service = service.clone();

				async move {
					let resp = match inner_service.ready().await {
						Ok(srv) => match srv.call(req).await {
							Ok(r) => Ok(r),
							Err(_) => Err(Error::Tower),
						},
						Err(_) => Err(Error::Tower),
					};

					resp_tx.send(resp).ok();
				}
			})
			.await // keep it in-flight
		});
		Self { request_tx: tx }
	}

	pub async fn request(&self, req: Request) -> Result<Response, Error> {
		let (tx, rx) = oneshot::channel::<Result<Response, Error>>();

		self.request_tx
			.clone()
			.send((req, tx))
			.await
			.map_err(|_| Error::Tower)?;

		rx.await.unwrap()
	}
}

pub static HTTP: Lazy<HttpClient> = Lazy::new(|| {
	let mut headers = reqwest::header::HeaderMap::new();

	#[cfg(test)]
	dotenvy::dotenv().ok();

	headers.insert(
		"API-Key",
		std::env::var("HYPIXEL_API_KEY")
			.expect("missing HYPIXEL_API_KEY")
			.parse()
			.expect("failed to parse HYPIXEL_API_KEY"),
	);

	let client = ClientBuilder::new(
		reqwest::Client::builder()
			.default_headers(headers)
			.build()
			.unwrap(),
	)
	.with(http_cache_reqwest::Cache(HttpCache {
		mode: CacheMode::Default,
		manager: MokaManager::new(
			CacheBuilder::new(10_000)
				.time_to_idle(Duration::from_secs(60 * 5))
				.time_to_live(Duration::from_secs(60 * 30))
				.build(),
		),
		options: None,
	}))
	.build();

	HttpClient::new(
		client,
		50_000_000,
		50_000_000,
		100,
		30,
		Duration::from_secs(30),
	)
});
