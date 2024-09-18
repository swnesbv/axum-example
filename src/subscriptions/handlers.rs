use axum::{
    extract::{State, Form},
    response::{Html, IntoResponse, Redirect},
    Extension,
};

use axum_extra::TypedHeader;
use headers::Cookie;

use sqlx::postgres::PgPool;

use tera::Context;

use rand::distributions::{Alphanumeric, DistString};

use crate::{
    common::Templates,
    auth,
    subscriptions::models::{FormResolution, FormSsc},
    subscriptions::repository::{
        all_groups, ssc_owner, ssc_to_user
    },
    subscriptions::views::{
        resolution_group, resolution_user, insert_ssc_group
    },
};


pub async fn get_owner_ssc(
    State(pool): State<PgPool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let token = auth::views::request_user(cookie).await;
    let cls = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(_) => return Err(Redirect::to("/account/login").into_response()),
    };

    let all = ssc_owner(pool, cls.id).await.unwrap();

    let mut context = Context::new();
    context.insert("all", &all);
    Ok(Html(templates.render("ssc_owner", &context).unwrap()))
}

pub async fn get_to_user(
    State(pool): State<PgPool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let token = auth::views::request_user(cookie).await;
    let cls = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(_) => return Err(Redirect::to("/account/login").into_response()),
    };

    let all = ssc_to_user(pool, cls.id).await.unwrap();

    let mut context = Context::new();
    context.insert("all", &all);
    Ok(Html(templates.render("ssc_to_user", &context).unwrap()))
}

pub async fn post_resolution_user(
    State(pool): State<PgPool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Form(form): Form<FormResolution>
) -> impl IntoResponse {

    let _ = auth::views::request_token(cookie).await.unwrap();

    let id = form.id;
    let dialogue = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let _ = resolution_user(pool, id, dialogue.clone()).await;

    Redirect::to(&("/chat-room/room/".to_owned() + &dialogue)).into_response()
}


pub async fn get_groups(
    State(pool): State<PgPool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let token = auth::views::request_user(cookie).await;
    let cls = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(_) => return Err(Redirect::to("/account/login").into_response()),
    };

    let all = all_groups(pool).await.unwrap();

    let mut context = Context::new();
    context.insert("cls", &cls);
    context.insert("all", &all);
    Ok(Html(templates.render("groups", &context).unwrap()))
}

pub async fn post_ssc_group(
    State(pool): State<PgPool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Form(form): Form<FormSsc>
) -> impl IntoResponse {

    let i = auth::views::request_token(cookie).await.unwrap();
    let c: i32 = i.claims.id;

    let title = "ssc group";
    let description = "expr";
    let to_group = form.to_group;

    let _ = insert_ssc_group(
        pool, c, title, description, to_group.expect("REASON"), i
    ).await;

    Redirect::to("/subscriptions/groups").into_response()
}

pub async fn post_resolution_group(
    State(pool): State<PgPool>,
    Form(form): Form<FormResolution>
) -> impl IntoResponse {

    let id = form.id;
    let to_group = form.to_group;

    let _ = resolution_group(pool, id, to_group.expect("REASON")).await;

    Redirect::to("/subscriptions/group").into_response()
}