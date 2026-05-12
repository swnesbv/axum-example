use std::sync::Arc;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
    http::header::{HeaderMap},
    Extension,
};
use tera::Context;

use crate::{
    common::Templates,
    auth::models::AuthRedis,
    photo::repository::{sl_photo, zip_collection}
};

pub async fn get_collections(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();
    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            context.insert("err", &err.to_string());
            return Err(Html(templates.render("collections", &context).unwrap()));
        }
        Ok(None) | Err(None) => {
            context.insert("is_no", "Caramba bullfighting and damn it");
            return Err(Html(templates.render("collections", &context).unwrap()))
        }
    };
    let all = zip_collection(i.pool.clone(), t.id).await.unwrap();

    context.insert("all", &all);
    context.insert("t", &t);
    Ok(Html(templates.render("collections", &context).unwrap()))
}


pub async fn get_slider_photo(
    Path(id): Path<i32>,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let user = sl_photo(i.pool.clone(), id).await;
    match user {
        Ok(expr) => {
            context.insert("i", &expr);
            Ok(Html(templates.render("slider_photo", &context).unwrap()))
        }
        Err(Some(err)) => {
            context.insert("err", &err.to_string());
            Err(Html(templates.render("slider_photo", &context).unwrap()))
        }
        Err(None) => {
            context.insert("is_no", "Caramba bullfighting and damn it");
            Err(Html(templates.render("slider_photo", &context).unwrap()))
        }
    }
}


