use std::sync::Arc;
use axum::{
    body::Body,
    extract::{Form, State, Path},
    http::{Response, StatusCode},
    http::{header::{HeaderMap}},
    response::{Html, IntoResponse, Redirect},
    Extension,
};
use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Pbkdf2,
};
use tera::Context;

use crate::{
    auth::models::{AuthRedis},
    common::Templates,
    profile::models::{
        FormNewUser, FormPasswordChange, FormUpdateUser
    },
    profile::views::{update_details, del_admin, del_user},
    provision::models::ParsePointError
};

pub async fn get_signup(
    Extension(templates): Extension<Templates>
) -> impl IntoResponse {

    Html(templates.render("signup", &Context::new()).unwrap())
}

pub async fn post_signup(
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
    Form(form): Form<FormNewUser>,
) -> impl IntoResponse {

    let mut context = Context::new();
    // ..
    let pg = match i.pool.get().await{
        Ok(expr) => expr,
        Err(err) => {
            context.insert("err", &err.to_string());
            return Err(Html(templates.render("signup", &context).unwrap()))
        }
    };
    let q_email = pg.query_one(
        "SELECT email FROM users WHERE email=$1",
        &[&form.email]
    )
    .await;
    let _ = match q_email {
        Ok(_) => {
            context.insert("err", "email already exists..");
            Ok(Html(templates.render("signup", &context).unwrap()))
        }
        Err(err) => {
            context.insert("err", &err.to_string());
            Err(Html(templates.render("signup", &context).unwrap()))
        }
    };
    let q_name = pg.query_one(
        "SELECT username FROM users WHERE username=$1",
        &[&form.username]
    )
    .await;
    let _ = match q_name {
        Ok(_) => {
            context.insert("err", "username already exists..");
            Ok(Html(templates.render("signup", &context).unwrap()))
        }
        Err(err) => {
            context.insert("err", &err.to_string());
            Err(Html(templates.render("signup", &context).unwrap()))
        }
    };
    // ..

    let salt = SaltString::generate(&mut OsRng);
    let pass = Pbkdf2.hash_password(form.password.as_bytes(), &salt);
    let hashed_password = match pass {
        Ok(pass) => pass.to_string(),
        Err(_) => "Err password".to_string(),
    };
    let status: Vec<String> = vec![];
    let _ = pg.execute(
        "INSERT INTO users (email, username, password, status, created_at) VALUES ($1,$2,$3,$4,now())",
        &[&form.email, &form.username, &hashed_password, &status]
    )
    .await
    .unwrap();
    Ok(Redirect::to("/account/users").into_response())
}

pub async fn get_update_user(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            context.insert("err", &err);
            return Err(Html(templates.render("update", &context).unwrap()))
        }
        Ok(None) | Err(None) => {
            context.insert("is_no", "Caramba bullfighting and damn it");
            return Err(Html(templates.render("update", &context).unwrap()))
        }
    };

    let user = update_details(i.pool.clone(), t.id).await;
    match user {
        Ok(user) => {
            context.insert("i", &user);
            Ok(Html(templates.render("update", &context).unwrap()))
        }
        Err(Some(err)) => {
            context.insert("err", &err.to_string());
            Err(Html(templates.render("update", &context).unwrap()))
        }
        Err(None) => {
            context.insert("is_no", "Caramba bullfighting and damn it");
            Err(Html(templates.render("update", &context).unwrap()))
        }
    }
}

pub async fn post_update_user(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
    Form(form): Form<FormUpdateUser>,
) -> impl IntoResponse {

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Ok(None) | Err(Some(_)) => return Ok(Redirect::to("/account/login").into_response()),
        Err(None) => return Ok(Redirect::to("/account/login").into_response()),
    };
    let pg = match i.pool.get().await{
        Ok(expr) => expr,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err", &err.to_string());
            return Err(Html(templates.render("update", &context).unwrap()))
        }
    };
    let _ = pg.execute(
        "UPDATE users SET email=$2, username=$3, updated_at=now() WHERE id=$1",
        &[&t.id, &form.email, &form.username]
    )
    .await
    .unwrap();
    Ok(Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", "/account/login")
        .header(
            "Set-Cookie",
            format!(
                "{}={}; Path={}; HttpOnly={}; SameSite={}",
                "visit", "_", "/", "true", "lax",
            ),
        )
        .body(Body::from("not found"))
        .unwrap())
}


pub async fn get_password_change(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            context.insert("err", &err);
            return Err(Html(templates.render("password_change", &context).unwrap()))
        }
        Ok(None) | Err(None) => {
            context.insert("is_no", "Caramba bullfighting and damn it");
            return Err(Html(templates.render("password_change", &context).unwrap()))
        }
    };
    let user = update_details(i.pool.clone(), t.id).await;
    match user {
        Ok(expr) => {
            context.insert("user", &expr);
            Ok(Html(templates.render("password_change", &context).unwrap()))
        }
        Err(Some(err)) => {
            context.insert("err", &err.to_string());
            Ok(Html(templates.render("password_change", &context).unwrap()))
        }
        Err(None) => {
            context.insert("is_no", "Caramba bullfighting and damn it");
            Err(Html(templates.render("password_change", &context).unwrap()))
        }
    }
}

pub async fn post_password_change(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
    Form(form): Form<FormPasswordChange>,
) -> impl IntoResponse {

    let mut context = Context::new();

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Ok(None) | Err(Some(_)) => return Ok(Redirect::to("/account/login").into_response()),
        Err(None) => return Ok(Redirect::to("/account/login").into_response()),
    };
    let pg = match i.pool.get().await{
        Ok(expr) => expr,
        Err(err) => {
            context.insert("err", &err.to_string());
            return Err(Html(templates.render("password_change", &context).unwrap()))
        }
    };

    let salt = SaltString::generate(&mut OsRng);
    let pass = Pbkdf2.hash_password(form.password.as_bytes(), &salt);
    let hashed_password = match pass {
        Ok(expr) => expr.to_string(),
        Err(_) => "Err password".to_string(),
    };
    let _ = pg.execute(
        "UPDATE users SET password=$2, updated_at=now() WHERE id=$1",
        &[&t.id, &hashed_password]
    )
    .await
    .unwrap();

    Ok(Redirect::to("/account/login").into_response())
}

pub async fn get_del_user(
    headers: HeaderMap,
    Path(uaer_id): Path<String>,
    State(i): State<Arc<AuthRedis>>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Ok(None) | Err(Some(_)) => return Err(Redirect::to("/account/login").into_response()),
        Err(None) => return Err(Redirect::to("/account/login").into_response()),
    };
    let number = match uaer_id.parse::<i32>().map_err(|_| ParsePointError) {
        Ok(expr) => expr,
        Err(_) => return Err(Redirect::to("/account/login").into_response())
    };

    if t.status.contains(&"admin".to_owned()) {
        match del_admin(i.pool.clone(), number).await {
            Ok(res) => res,
            Err(_) => return Err(Redirect::to("/account/login").into_response())
        };
    } else {
        match del_user(i.pool.clone(), number, t.email).await {
            Ok(res) => res,
            Err(_) => return Err(Redirect::to("/account/login").into_response())
        };
    }
    Ok(Redirect::to("/account/users").into_response())
}