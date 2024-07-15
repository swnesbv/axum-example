use sqlx::postgres::PgPool;

use axum::{
    body::Body,
    extract::{Form, State},
    http::{Response, StatusCode},
    response::{Html, IntoResponse, Redirect},
    Extension,
};
use chrono::Utc;

use tera::Context;

use axum_extra::TypedHeader;
use headers::Cookie;

use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Pbkdf2,
};

use crate::auth;
use crate::{
    common::{Templates},
    profile::models::{
        NewUser,
        UpdateUser,
        FormNewUser,
        FormUpdateUser,
        FormPasswordChange, PasswordChange,
    },
    profile::views::{update_details},
};


pub async fn get_signup(Extension(templates): Extension<Templates>) -> impl IntoResponse {
    Html(templates.render("signup", &Context::new()).unwrap())
}


pub async fn post_signup(
    State(pool): State<PgPool>,
    Extension(templates): Extension<Templates>,
    Form(form): Form<FormNewUser>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();
    // ..
    let q_email = sqlx::query!(
        "SELECT email FROM users WHERE email=$1", form.email.clone()
        )
        .fetch_optional(&pool).await;
        println!("email..{:?}", q_email);
    match q_email {
        Ok(None) => (),
        _ => {
            context.insert("for_email", "email already exists..");
            return Err(Html(templates.render("signup", &context).unwrap()));
        },
    };
    // ..
    let q_name = sqlx::query!(
        "SELECT username FROM users WHERE username=$1", form.username.clone()
        )
        .fetch_optional(&pool).await;
        println!("email..{:?}", q_name);
    match q_name {
        Ok(None) => (),
        _ => {
            context.insert("for_username", "username already exists..");
            return Err(Html(templates.render("signup", &context).unwrap()));
        },
    };
    // ..

    let salt = SaltString::generate(&mut OsRng);
    let pass = Pbkdf2.hash_password(form.password.as_bytes(), &salt);
    let hashed_password = match pass {
        Ok(pass) => pass.to_string(),
        Err(_) => "Err password".to_string(),
    };

    let u = NewUser {
        email: form.email.clone(),
        username: form.username.clone(),
        password: hashed_password,
        created_at: Utc::now(),
    };
    let _ = sqlx::query(
        "INSERT INTO users (email, username, password, created_at) VALUES ($1,$2,$3,$4)"
        )
        .bind(&u.email)
        .bind(&u.username)
        .bind(&u.password)
        .bind(u.created_at)
        .execute(&pool)
        .await
        .unwrap();

    Ok(Redirect::to("/account/users").into_response())
}


pub async fn get_update(
    State(pool): State<PgPool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let token = auth::views::err_user(TypedHeader(cookie)).await;
    let t = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(err) => return Ok(
            {
                context.insert("err_token", &err);
                Html(templates.render("update", &context).unwrap())
            }
        ),
    };

    let user = update_details(pool, t.id).await;

    match user {
        Ok(user) => {
            context.insert("user", &user);
            Ok(Html(templates.render("update", &context).unwrap()))
        },
        Err(err) => {
            context.insert("not_details", &err);
            Ok(Html(templates.render("update", &context).unwrap()))
        },
    }

}


pub async fn post_update_user(
    State(pool): State<PgPool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Form(form): Form<FormUpdateUser>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let token = auth::views::request_user(TypedHeader(cookie)).await;

    let t = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(_) => return Err(Redirect::to("/account/login").into_response()),
    };

    let u = UpdateUser {
        email: form.email.clone(),
        username: form.username.clone(),
        updated_at: Some(Utc::now()),
    };

    let _ = sqlx::query_as!(UpdateUser, "UPDATE users SET email=$2, username=$3, updated_at=$4 WHERE id=$1", t.id, u.email, u.username, u.updated_at)
        .fetch_one(&pool)
        .await;

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
    State(pool): State<PgPool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let token = auth::views::err_user(TypedHeader(cookie)).await;
    let t = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(err) => return Ok(
            {
                context.insert("err_token", &err);
                Html(templates.render("update", &context).unwrap())
            }
        ),
    };

    let user = update_details(pool, t.id).await;

    match user {
        Ok(user) => {
            context.insert("user", &user);
            Ok(Html(templates.render("password_change", &context).unwrap()))
        },
        Err(err) => {
            context.insert("not_details", &err);
            Ok(Html(templates.render("password_change", &context).unwrap()))
        },
    }
}


pub async fn post_password_change(
    State(pool): State<PgPool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Form(form): Form<FormPasswordChange>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let token = auth::views::request_user(TypedHeader(cookie)).await;

    let t = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(_) => return Err(Redirect::to("/account/login").into_response()),
    };

    let salt = SaltString::generate(&mut OsRng);
    let pass = Pbkdf2.hash_password(form.password.as_bytes(), &salt);
    let hashed_password = match pass {
        Ok(pass) => pass.to_string(),
        Err(_) => "Err password".to_string(),
    };

    let u = PasswordChange {
        password: hashed_password,
        updated_at: Some(Utc::now()),
    };

    let _ = sqlx::query_as!(PasswordChange, "UPDATE users SET password=$2, updated_at=$3 WHERE id=$1", t.id, u.password, u.updated_at)
        .fetch_one(&pool)
        .await;

    Ok(Redirect::to("/account/login").into_response())
}
