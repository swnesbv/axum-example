use sqlx::postgres::PgPool;

use axum::{
    extract::{State, Form},
    response::{Html, IntoResponse, Redirect},
    Extension,
};

use chrono::{Utc};

use headers::Cookie;
use axum_extra::TypedHeader;

use tera::Context;

use crate::{
    common::{Templates},
    auth,
    subscriptions::models::{FormGroup, FormSsc},
    subscriptions::views::{insert_ssc_user, insert_ssc_group, },
};


pub async fn post_ssc_user(
    State(pool): State<PgPool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Form(form): Form<FormSsc>
) -> impl IntoResponse {

    let token = auth::views::request_token(cookie).await.unwrap();

    let title = "ssc user";
    let description = "expr";
    let to_user = form.to_user;

    let _ = insert_ssc_user(
        pool, token.claims.id, title, description, to_user.expect("REASON"), token
    ).await;

    Redirect::to("/subscriptions/ssc-owner").into_response()
}


pub async fn get_creat_group(
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>
) -> Result<impl IntoResponse, impl IntoResponse> {

    let token = auth::views::request_user(cookie).await;
    match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(_) => return Err(Redirect::to("/account/login").into_response()),
    };

    Ok(Html(templates.render("creat", &Context::new()).unwrap()))
}

pub async fn post_creat_group(
    State(pool): State<PgPool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
    Form(form): Form<FormGroup>
) -> impl IntoResponse {

    let token = auth::views::request_token(cookie).await.unwrap();

    let result = sqlx::query(
        "INSERT INTO provision_d (user_id, title, description, created_at) VALUES ($1,$2,$3,$4)"
        )
        .bind(token.claims.id)
        .bind(&form.title)
        .bind(&form.description)
        .bind(Utc::now())
        .execute(&pool)
        .await;
    match result {
        Ok(result) => result,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err_token", &err.to_string());
            return Err(Html(templates.render("creat", &context).unwrap()));
        }
    };
    Ok(Redirect::to("/subscriptions/groups").into_response())

}

pub async fn post_ssc_group(
    State(pool): State<PgPool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Form(form): Form<FormSsc>
) -> impl IntoResponse {

    let token = auth::views::request_token(cookie).await.unwrap();

    let title = "ssc group";
    let description = "expr";
    let to_group = form.to_group;

    let _ = insert_ssc_group(
        pool, token.claims.id, title, description, to_group.expect("REASON"), token
    ).await;

    Redirect::to("/subscriptions/groups").into_response()
}
