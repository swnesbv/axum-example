use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use axum::{
	body::Body,
	extract::{Form, State},
	http::{Response, StatusCode},
	http::header::{HeaderMap},
	response::{Html, IntoResponse},
	Extension,
};
use redis::{AsyncCommands};
use rand::distr::{Alphanumeric, SampleString};
use pbkdf2::{
	password_hash::{PasswordHash, PasswordVerifier},
	Pbkdf2,
};
use jwt_simple::prelude::*;
use tera::Context;

use crate::{
	auth::views::{read_msg},
	auth::models::{
		KeyEmail, AuthRedis, AuToken, FormLogin,
	},
	common::Templates,
};

pub async fn get_login(
	cookie: HeaderMap,
	Extension(templates): Extension<Templates>,
) -> impl IntoResponse {

	let msg = read_msg(cookie).await.unwrap();

	let mut context = Context::new();
	context.insert("msg", &msg.unwrap());
	Html(templates.render("login", &context).unwrap())
}

pub async fn post_login(
	State(i): State<Arc<AuthRedis>>,
	Extension(templates): Extension<Templates>,
	Form(f): Form<FormLogin>,
) -> Result<impl IntoResponse, impl IntoResponse> {

	let mut context = Context::new();

    let pg = match i.pool.get().await{
        Ok(expr) => expr,
        Err(err) => {
            context.insert("err", &err.to_string());
            return Err(Html(templates.render("login", &context).unwrap()))
        }
    };

	let email = pg.query(
		"SELECT email FROM users WHERE email=$1;",
		&[&f.email]
	)
	.await
	.unwrap();
	if email.is_empty() {
		context.insert("for_email", "this email is not available..!");
		return Err(Html(templates.render("login", &context).unwrap()));
	}
	let pass = pg.query_one(
		"SELECT password FROM users WHERE email=$1;", &[&f.email]
	)
	.await
	.unwrap();
	let rpass: &str = pass.get(0);
	let parsed_hash = PasswordHash::new(rpass).unwrap();
	let veri = Pbkdf2
		.verify_password(f.password.as_bytes(), &parsed_hash)
		.is_ok();
	if !veri {
		context.insert("for_password", "password is not correct..!");
		return Err(Html(templates.render("login", &context).unwrap()));
	};

	let in_user = pg.query(
		"SELECT id, email, username, status FROM users WHERE email=$1", &[&f.email]
	)
	.await;
	let user = match in_user {
		Ok(expr) => expr,
		Err(err) => {
			context.insert("err", &err.to_string());
			return Err(Html(templates.render("login", &context).unwrap()))
		}
	};
	// ..Token
	let row = &user[0];
	let obj = AuToken {
		id: row.get(0),
		email: row.get(1),
		username: row.get(2),
		status: row.get(3),
	};
	//..
	let de_key = RsaOaepDecryptionKey::generate(2048).unwrap();
	let en_key = de_key.encryption_key();
    let dialogue = Alphanumeric.sample_string(
        &mut rand::rng(), 12
    );
    //..
    let sess = Alphanumeric.sample_string(
        &mut rand::rng(), 8
    );
    //..

    if fs::exists(
    	"./static/de_key/user/".to_string() + &f.email
    ).unwrap() {
    	fs::remove_dir_all("./static/de_key/user/".to_string() + &f.email).unwrap();
    }
    let _ = fs::create_dir_all(
    	"./static/de_key/user/".to_string() + &f.email
    );
    let mut buffer = File::create(
    	format!("./static/de_key/user/{}/{}{}", f.email, dialogue, ".der")
    ).unwrap();
    let _ = buffer.write_all(&de_key.to_der().unwrap());

    // Redis..!
    let mut rs = match i.conn.get().await {
        Ok(expr) => expr,
        Err(err) => {
        	context.insert("err", &err.to_string());
        	return Err(Html(templates.render("login", &context).unwrap()))
        }
    };
    let key = KeyEmail {
    	key:   dialogue,
        email: f.email,
    };
    let session = rs.set_ex::<&str, &str, ()>(
    	&sess,
    	&serde_json::to_string(&key).unwrap(),
    	7200
    	).await;
    match session {
        Ok(expr) => expr,
        Err(err) => {
        	context.insert("err", &err.to_string());
        	return Err(Html(templates.render("login", &context).unwrap()))
        }
    };
    //..

	let claims = Claims::with_custom_claims(
		obj, Duration::from_hours(2)
	);
	let token = en_key.encrypt(claims).unwrap();

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
		.header(
			"Set-Cookie",
			format!(
				"{}={}; Path={}; HttpOnly={}; SameSite={}",
				"sess", sess, "/", "true", "lax",
			),
		)
		.body(Body::from("not found")).unwrap()
	)
}
