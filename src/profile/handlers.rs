use sqlx::postgres::PgPool;
use axum::{
    extract::{State, Path},
    response::{Html, IntoResponse},
    Extension,
};

use tera::Context;

use axum_extra::TypedHeader;
use headers::Cookie;

use crate::{
    common::{Templates},
    auth::views::{request_user},
    profile::views::{all, details},
    // profile::models::{ListUser},
};


pub async fn index(
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    
    let mut context = Context::new();

    let token = request_user(TypedHeader(cookie)).await;

    match token {
        Ok(Some(token)) => {
            context.insert("token", &token);
            Ok(Html(templates.render("index", &context).unwrap()))
        }
        Ok(None) => {
            context.insert("not_user", "unauthorized");
            Err(Html(templates.render("index", &context).unwrap()))
        }
        Err(err) => {
            context.insert("not_user", &err.expect("REASON").to_string());
            Err(Html(templates.render("index", &context).unwrap()))
        }
    }
}


pub async fn users(
    State(pool): State<PgPool>,
    Extension(templates): Extension<Templates>,
) -> impl IntoResponse {
    
    let users = all(pool).await.unwrap();

    let mut context = Context::new();
    context.insert("users", &users);
    Html(templates.render("users", &context).unwrap())
}

pub async fn user(
    Path(name): Path<String>,
    State(pool): State<PgPool>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let user = details(pool, name).await;

    let mut context = Context::new();

    match user {
        Ok(user) => {
            context.insert("user", &user);
            Ok(Html(templates.render("user", &context).unwrap()))
        },
        Err(err) => {
            context.insert("not_details", &err);
            Err(Html(templates.render("user", &context).unwrap()))
        }
    }
}
