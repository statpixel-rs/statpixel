use std::sync::Arc;

use api::{http::HTTP, Player};
use axum::{
	extract::{FromRequestParts, Json, State},
	headers::{authorization, Authorization},
	http::{header, request::Parts, StatusCode},
	response::{IntoResponse, Response},
	RequestPartsExt, TypedHeader,
};
use axum_extra::extract::WithRejection;
use chrono::{DateTime, Duration, Utc};
use database::{models::User, schema::user};
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use once_cell::sync::Lazy;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use translate::context;

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
	pub exp: usize,
	pub iat: usize,
	pub iss: String,
	pub id: u64,
	pub name: String,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum AuthError {
	WrongCredentials,
	MissingCredentials,
}

impl IntoResponse for AuthError {
	fn into_response(self) -> Response {
		let (status, message) = match self {
			AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "wrong credentials"),
			AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "missing credentials"),
		};

		(
			status,
			Json(super::error::ErrorMessage {
				success: false,
				origin: "with_rejection",
				message: message.to_string(),
			}),
		)
			.into_response()
	}
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for Claims
where
	S: Send + Sync,
{
	type Rejection = AuthError;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
		// Extract the token from the authorization header
		let TypedHeader(Authorization(bearer)) = parts
			.extract::<TypedHeader<Authorization<authorization::Bearer>>>()
			.await
			.map_err(|_| AuthError::MissingCredentials)?;

		// Decode the user data
		let token_data =
			jsonwebtoken::decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
				.map_err(|_| AuthError::WrongCredentials)?;

		Ok(token_data.claims)
	}
}

#[derive(Deserialize, Debug)]
pub struct Code {
	pub code: String,
}

#[derive(Deserialize)]
pub struct Access {
	pub access_token: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
	pub token: String,
	pub expires: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct DiscordUser {
	pub id: String,
	pub username: String,
	pub discriminator: String,
}

static URL: Lazy<Url> =
	Lazy::new(|| Url::parse("https://discord.com/api/v10/oauth2/token").unwrap());

static KEYS: Lazy<Keys> = Lazy::new(|| {
	#[cfg(not(feature = "runtime_env"))]
	let secret = dotenvy_macro::dotenv!("JWT_SECRET").as_bytes();

	#[cfg(feature = "runtime_env")]
	let secret = std::env::var("JWT_SECRET")
		.expect("JWT_SECRET not set")
		.as_bytes();

	Keys::new(secret)
});

struct Keys {
	encoding: EncodingKey,
	decoding: DecodingKey,
}

impl Keys {
	fn new(secret: &[u8]) -> Self {
		Self {
			encoding: EncodingKey::from_secret(secret),
			decoding: DecodingKey::from_secret(secret),
		}
	}
}

#[derive(Serialize)]
pub struct MeResponse {
	pub background: Option<u32>,
	pub is_owner: bool,
	pub uuid: Option<uuid::Uuid>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

#[allow(clippy::unused_async)]
pub async fn me(
	State(state): State<Arc<super::Data>>,
	claims: super::auth::Claims,
) -> Result<impl IntoResponse, StatusCode> {
	let ctx = context::Context::external(&state);
	let user = user::table
		.filter(user::id.eq(claims.id as i64))
		.select(User::as_select())
		.get_result::<User>(
			&mut state
				.pool
				.get()
				.await
				.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
		)
		.await
		.map_err(|_| StatusCode::NOT_FOUND)?;

	let is_owner = if let Some(uuid) = user.uuid {
		let player = Player::from_uuid_unchecked(uuid);
		let data = player
			.get_data(&ctx)
			.await
			.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

		Some(claims.name) == data.socials.discord
	} else {
		false
	};

	#[allow(clippy::cast_sign_loss)]
	Ok(Json(MeResponse {
		uuid: user.uuid,
		is_owner,
		background: user.colour.map(|c| c as u32),
		created_at: user.created_at,
		updated_at: user.updated_at,
	}))
}

#[derive(Deserialize)]
pub struct MeUpdate {
	pub uuid: Option<uuid::Uuid>,
	pub background: Option<u32>,
}

pub async fn update_me(
	State(state): State<Arc<super::Data>>,
	claims: super::auth::Claims,
	WithRejection(Json(update), _): super::extract::Json<MeUpdate>,
) -> Result<impl IntoResponse, StatusCode> {
	diesel::insert_into(user::table)
		.values((
			user::colour.eq(update.background.map(|b| b as i32)),
			user::uuid.eq(update.uuid),
			user::id.eq(claims.id as i64),
		))
		.on_conflict(user::id)
		.do_update()
		.set((
			user::colour.eq(update.background.map(|b| b as i32)),
			user::uuid.eq(update.uuid),
			user::id.eq(claims.id as i64),
		))
		.execute(
			&mut state
				.pool
				.get()
				.await
				.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
		)
		.await
		.map_err(|_| StatusCode::NOT_FOUND)?;

	Ok(StatusCode::OK)
}

pub async fn login(
	WithRejection(query, _): super::extract::Query<Code>,
) -> Result<impl IntoResponse, StatusCode> {
	#[cfg(not(feature = "runtime_env"))]
	#[allow(non_upper_case_globals)]
	const client_id: &str = dotenvy_macro::dotenv!("CLIENT_ID");

	#[cfg(feature = "runtime_env")]
	let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID not set");

	#[cfg(not(feature = "runtime_env"))]
	#[allow(non_upper_case_globals)]
	const client_secret: &str = dotenvy_macro::dotenv!("CLIENT_SECRET");

	#[cfg(feature = "runtime_env")]
	let client_secret = std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set");

	#[cfg(not(feature = "runtime_env"))]
	#[allow(non_upper_case_globals)]
	const redirect_uri: &str = dotenvy_macro::dotenv!("REDIRECT_URI");

	#[cfg(feature = "runtime_env")]
	let redirect_uri = std::env::var("REDIRECT_URI").expect("REDIRECT_URI not set");

	let access = HTTP
		.post(URL.clone())
		.form(&[
			("client_id", client_id),
			("client_secret", client_secret),
			("grant_type", "authorization_code"),
			("code", &query.code),
			("redirect_uri", redirect_uri),
			("scope", "identify"),
		])
		.send()
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
		.json::<Access>()
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	let user = HTTP
		.get("https://discord.com/api/v10/users/@me".try_into().unwrap())
		.header(
			reqwest::header::AUTHORIZATION,
			format!("Bearer {}", access.access_token),
		)
		.send()
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
		.json::<DiscordUser>()
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	let id = user
		.id
		.parse::<u64>()
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	let token = create_token(
		id,
		if user.discriminator == "0" {
			user.username
		} else {
			format!("{}#{}", user.username, user.discriminator)
		},
	);

	Ok((
		StatusCode::PERMANENT_REDIRECT,
		[(
			header::LOCATION,
			format!("https://statpixel.xyz?token={token}"),
		)],
	))
}

pub fn create_token(id: u64, name: String) -> String {
	let now = Utc::now();

	#[allow(clippy::cast_possible_truncation)]
	#[allow(clippy::cast_sign_loss)]
	let claims = Claims {
		exp: (now + Duration::days(30)).timestamp() as usize,
		iat: now.timestamp() as usize,
		iss: "https://statpixel.xyz".to_string(),
		id,
		name,
	};

	jsonwebtoken::encode(
		&jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
		&claims,
		&KEYS.encoding,
	)
	.unwrap()
}
