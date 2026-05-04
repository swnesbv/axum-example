use std::sync::Arc;
use axum::{
    extract::{State, Form},
    response::{Html, IntoResponse, Redirect},
    http::{header::{HeaderMap}},
    Extension,
};
use tera::Context;
use rand::distr::{Alphanumeric, SampleString};

use crate::{
    common::Templates,
    auth::models::{AuthRedis},
    subscriptions::models::{FormResolution, FormSsc},
    subscriptions::repository::{
        all_groups, ssc_owner, ssc_to_user
    },
    subscriptions::views::{
        resolution_group, resolution_user, insert_ssc_group
    },
};


pub async fn get_owner_ssc(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            context.insert("err", &err);
            return Err(Html(templates.render("ssc_owner", &context).unwrap()))
        }
        Ok(None) | Err(None) => {
            context.insert("is_no", "Caramba bullfighting and damn it");
            return Err(Html(templates.render("ssc_owner", &context).unwrap()))
        }
    };

    let all = ssc_owner(i.pool.clone(), t.id).await.unwrap();

    context.insert("all", &all);
    Ok(Html(templates.render("ssc_owner", &context).unwrap()))
}

pub async fn get_to_user(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            context.insert("err", &err);
            return Err(Html(templates.render("ssc_to_user", &context).unwrap()))
        }
        Ok(None) | Err(None) => {
            context.insert("is_no", "Caramba bullfighting and damn it");
            return Err(Html(templates.render("ssc_to_user", &context).unwrap()))
        }
    };

    let all = ssc_to_user(i.pool.clone(), t.id).await.unwrap();

    context.insert("all", &all);
    Ok(Html(templates.render("ssc_to_user", &context).unwrap()))
}

pub async fn post_resolution_user(
    State(i): State<Arc<AuthRedis>>,
    Form(form): Form<FormResolution>
) -> impl IntoResponse {

    let id = form.id;

    let dialogue = Alphanumeric.sample_string(
        &mut rand::rng(), 16
    );

    let _ = resolution_user(i.pool.clone(), id, dialogue.clone()).await;
    Redirect::to(
        &("/chat-room/room/".to_owned() + &dialogue)
    ).into_response()
}


pub async fn get_groups(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            context.insert("err", &err);
            return Err(Html(templates.render("groups", &context).unwrap()))
        }
        Ok(None) | Err(None) => {
            context.insert("is_no", "Caramba bullfighting and damn it");
            return Err(Html(templates.render("groups", &context).unwrap()))
        }
    };

    let all = all_groups(i.pool.clone()).await.unwrap();

    context.insert("cls", &t);
    context.insert("all", &all);
    Ok(Html(templates.render("groups", &context).unwrap()))
}

pub async fn post_ssc_group(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Form(form): Form<FormSsc>
) -> impl IntoResponse {

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Ok(None) | Err(Some(_)) => return Err(Redirect::to("/account/login").into_response()),
        Err(None) => return Err(Redirect::to("/account/login").into_response()),
    };

    let title = "ssc group";
    let description = "expr";
    let to_group = form.to_group.unwrap();

    let _ = insert_ssc_group(
        i.pool.clone(), t.id, title, description, to_group, t.clone()
    ).await;

    Ok(Redirect::to("/subscriptions/groups").into_response())
}

pub async fn post_resolution_group(
    State(i): State<Arc<AuthRedis>>,
    Form(form): Form<FormResolution>
) -> impl IntoResponse {

    let id = form.id;
    let to_group = form.to_group.unwrap();

    let _ = resolution_group(i.pool.clone(), id, to_group).await;

    Redirect::to("/subscriptions/group").into_response()
}