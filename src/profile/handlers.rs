use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
    Extension
};
use sqlx::postgres::PgPool;

use tera::Context;

use axum_extra::TypedHeader;
use headers::Cookie;

use crate::{
    auth::views::request_user,
    common::{DatabaseConn, Templates},
    profile::views::{all, details},
    comments::views::{i_comments},
};


pub async fn index(
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let i = public_ip_address::perform_lookup(None).await.unwrap();
    println!(" i.. {:#?}", i);

    let i_city = i.city;
    let i_region = i.region;
    let i_country = i.country;
    let i_latitude = i.latitude;
    let i_longitude = i.longitude;

    let mut context = Context::new();

    context.insert("i_city", &i_city);
    context.insert("i_region", &i_region);
    context.insert("i_country", &i_country);
    context.insert("i_latitude", &i_latitude);
    context.insert("i_longitude", &i_longitude);

    let _ = Html(templates.render("index", &context).unwrap());

    let token = request_user(cookie).await;
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
    DatabaseConn(mut conn): DatabaseConn,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let user = details(&mut conn, name).await;

    let i = &("users-".to_owned() + &user.to_owned().unwrap().username);
    let comments = i_comments(&mut conn, i).await.unwrap();

    let mut context = Context::new();
    match user {
        Ok(user) => {
            context.insert("user", &user);
            context.insert("comments", &comments);
            Ok(Html(templates.render("user", &context).unwrap()))
        }
        Err(err) => {
            context.insert("not_details", &err);
            Err(Html(templates.render("user", &context).unwrap()))
        }
    }
}
