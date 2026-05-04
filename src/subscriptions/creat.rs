use std::sync::Arc;
use axum::{
    extract::{State, Form},
    response::{Html, IntoResponse, Redirect},
    http::{header::{HeaderMap}},
    Extension,
};
use tera::Context;

use crate::{
    common::{Templates},
    auth::models::{AuthRedis},
    subscriptions::models::{FormGroup, FormSsc},
    subscriptions::views::{insert_ssc_user, insert_ssc_group, },
};

pub async fn post_ssc_user(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Form(form): Form<FormSsc>
) -> impl IntoResponse {

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Ok(None) | Err(Some(_)) => return Err(Redirect::to("/account/login").into_response()),
        Err(None) => return Err(Redirect::to("/account/login").into_response()),
    };
    let title = "title ssc user";
    let description = "description  ssc user";
    let to_user: i32 = form.to_user.unwrap();

    let _ = insert_ssc_user(
        i.pool.clone(), t.id, title, description, to_user, t
    ).await;
    Ok(Redirect::to("/subscriptions/ssc-owner").into_response())
}


pub async fn get_creat_group(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Ok(None) | Err(Some(_)) => return Err(Redirect::to("/account/login").into_response()),
        Err(None) => return Err(Redirect::to("/account/login").into_response()),
    };

    context.insert("t", &t);
    Ok(Html(templates.render("creat", &context).unwrap()))
}

pub async fn post_creat_group(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
    Form(form): Form<FormGroup>
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
            return Err(Html(templates.render("signup", &context).unwrap()))
        }
    };
    let result = pg.execute(
        "INSERT INTO provision_d (user_id, title, description, created_at) VALUES ($1,$2,$3,now())",
        &[&t.id, &form.title, &form.description]
        )
        .await;
    match result {
        Ok(result) => result,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err", &err.to_string());
            return Err(Html(templates.render("creat", &context).unwrap()));
        }
    };
    Ok(Redirect::to("/subscriptions/groups").into_response())

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
        i.pool.clone(), t.id, title, description, to_group, t
    ).await;

    Ok(Redirect::to("/subscriptions/groups").into_response())
}
