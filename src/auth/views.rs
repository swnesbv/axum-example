// use sqlx::postgres::PgPool;
// use axum::extract::State;

use headers::Cookie;

use jsonwebtoken::{errors, DecodingKey, TokenData};

use crate::auth::models::{
	Claims,
	// ListUser
};

pub async fn err_user(cookie: Cookie) -> Result<Option<Claims>, Option<String>> {
	let token = match cookie.get("visit") {
		Some(token) => token,
		None => return Ok(None),
	};

	let jwt_secret = dotenv::var("JWT_SECRET").expect("JWT SEC.. must be set");

	let decode_token = jsonwebtoken::decode::<Claims>(
		token,
		&DecodingKey::from_secret(jwt_secret.as_ref()),
		&jsonwebtoken::Validation::default(),
	);
	match decode_token {
		Ok(claims) => Ok(Some(claims.claims)),
		Err(err) => Err(Some(err.to_string())),
	}
}


pub async fn request_user(
	cookie: Cookie,
) -> Result<Option<Claims>, Option<errors::Error>> {

	let token = match cookie.get("visit") {
		Some(token) => token,
		None => return Ok(None),
	};

	let jwt_secret = dotenv::var("JWT_SECRET").expect("JWT SECRET must be set");

	let decode_token = jsonwebtoken::decode::<Claims>(
		token,
		&DecodingKey::from_secret(jwt_secret.as_ref()),
		&jsonwebtoken::Validation::default(),
	);
	match decode_token {
		Ok(claims) => Ok(Some(claims.claims)),
		Err(err) => Err(Some(err)),
	}
}

pub async fn request_token(
	cookie: Cookie,
) -> Result<TokenData<Claims>, Option<String>> {
	let token = match cookie.get("visit") {
		Some(token) => token,
		None => return Err(Some("Token not found".to_string())),
	};
	let jwt_secret = dotenv::var("JWT_SECRET").expect("JWT SECRET must be set");
	let decode_token = jsonwebtoken::decode::<Claims>(
		token,
		&DecodingKey::from_secret(jwt_secret.as_ref()),
		&jsonwebtoken::Validation::default(),
	);
	match decode_token {
		Ok(claims) => Ok(claims),
		Err(err) => Err(Some(err.to_string())),
	}
}

/*pub async fn request_auth(
	pool: PgPool,
	cookie: Cookie,
) -> Result<Option<ListUser>, Option<String>> {

	let claims = match request_token(cookie).await {
		Ok(claims) => claims,
		Err(_) => return Err(Some("token not found..!".to_string())),
	};

	let user = auth::repository::full_auth(State(pool), claims.claims.email).await;
	match user {
		Ok(user) => Ok(user),
		Err(_) => Err(Some("User not found".to_string())),
	}
}

pub async fn user_token(cookie: Cookie) -> Result<Option<Claims>, &'static str> {
	let token = match cookie.get("visit") {
		Some(token) => token,
		None => return Ok(None),
	};

	let jwt_secret = dotenv::var("JWT_SECRET").expect("JWT SECRET must be set");

	let decode_token = jsonwebtoken::decode::<Claims>(
		token,
		&DecodingKey::from_secret(jwt_secret.as_ref()),
		&jsonwebtoken::Validation::default(),
	)
	.unwrap();

	Ok(Some(decode_token.claims))
}*/
