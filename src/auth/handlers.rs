use sqlx::postgres::PgPool;
use sqlx::query;
use sqlx::query_as;

use axum::{
	body::Body,
	extract::{Form, State},
	http::{Response, StatusCode},
	response::{Html, IntoResponse},
	Extension,
};

use pbkdf2::{
	password_hash::{PasswordHash, PasswordVerifier},
	Pbkdf2,
};

use jsonwebtoken::{encode, EncodingKey, Header};

use axum_extra::TypedHeader;
use headers::Cookie;

use tera::Context;

use crate::{
	auth::models::{Claims, FormLogin, VeriUser},
	common::Templates,
	photo::views::read_msg,
};

pub async fn get_login(
	TypedHeader(cookie): TypedHeader<Cookie>,
	Extension(templates): Extension<Templates>,
) -> impl IntoResponse {

	let msg = read_msg(cookie).await.unwrap();

	let mut context = Context::new();
	context.insert("msg", &msg.unwrap());
	Html(templates.render("login", &context).unwrap())
}

pub async fn post_login(
	State(pool): State<PgPool>,
	Extension(templates): Extension<Templates>,
	Form(form): Form<FormLogin>,
) -> Result<impl IntoResponse, impl IntoResponse> {

	let mut context = Context::new();

	let email = query!(
		"SELECT email FROM users WHERE email=$1",
		&form.email
	)
	.fetch_optional(&pool)
	.await
	.unwrap();
	if email.is_none() {
		context.insert("for_email", "this email is not available..!");
		return Err(Html(templates.render("login", &context).unwrap()));
	}

	let pass = query!(
		"SELECT password FROM users WHERE email=$1",
		&form.email
	)
	.fetch_one(&pool)
	.await
	.unwrap();

	let parsed_hash = PasswordHash::new(pass.password.as_str()).unwrap();
	let veri = Pbkdf2
		.verify_password(form.password.as_bytes(), &parsed_hash)
		.is_ok();
	if !veri {
		context.insert("for_password", "password is not correct..!");
		return Err(Html(templates.render("login", &context).unwrap()));
	};

	let in_user = query_as!(
		VeriUser,
		"SELECT id, email, username, status FROM users WHERE email=$1",
		&form.email
	)
	.fetch_optional(&pool)
	.await
	.unwrap();

	let user = match in_user {
		Some(user) => user,
		None => return Err(Html(templates.render("login", &context).unwrap())),
	};

	let token: String = if user.status.contains(&"admin".to_owned()) {
		let claims = Claims {
			id: user.id,
			email: user.email,
			username: user.username,
			status: vec!["admin".to_owned()],
			exp: 10000000000,
		};
		let jwt_secret = dotenv::var("JWT_SECRET").expect("JWT SECRET must be set");
		encode(
			&Header::default(),
			&claims,
			&EncodingKey::from_secret(jwt_secret.as_ref()),
		).unwrap()
	} else {
		let claims = Claims {
			id: user.id,
			email: user.email,
			username: user.username,
			status: vec!["simply".to_owned()],
			exp: 10000000000,
		};
		let jwt_secret = dotenv::var("JWT_SECRET").expect("JWT SECRET must be set");
		encode(
			&Header::default(),
			&claims,
			&EncodingKey::from_secret(jwt_secret.as_ref()),
		).unwrap()
	};
	Ok(Response::builder()
		.status(StatusCode::FOUND)
		.header("Location", "/account/users")
		.header(
			"Set-Cookie",
			format!(
				"{}={}; Path={}; HttpOnly={}; SameSite={}",
				"visit", token, "/", "true", "lax",
			),
		)
		.body(Body::from("not found"))
		.unwrap())
}
