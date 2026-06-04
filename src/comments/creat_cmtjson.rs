use std::sync::Arc;
use axum::{
    extract::{Form, State, OriginalUri, Path},
    response::{IntoResponse, Redirect, Html},
    http::header::{HeaderMap},
    Extension
};
use tera::Context;

use crate::{
    common::Templates,
    auth::models::AuthRedis,
    comments::models::{FormJson, FormUpdateCmt},
    comments::cmtjson::{creat_cmt, all_cmt, id_cmt, update_cmt, del_cmt},
    photo::repository::{add_msg}
};

pub async fn get_creat_cmt(
    headers: HeaderMap,
    Path(to_id): Path<i32>,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> impl IntoResponse {

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Ok(None) | Err(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(Some(err)) => {
            return Err(
                    add_msg(
                        err.to_string(),
                        "danger".to_string(),
                        "/account/login".to_string(),
                ).await
            )
        }
    };
    let result = all_cmt(i.pool.clone(), to_id, "users").await.unwrap();

    let mut context = Context::new();
    context.insert("t", &t);
    context.insert("cmt", &result);
    Ok(Html(templates.render("cmtjson", &context).unwrap()))
}

pub async fn post_creat_cmt(
    headers: HeaderMap,
    Path(to_id): Path<i32>,
    State(i): State<Arc<AuthRedis>>,
    OriginalUri(original_uri): OriginalUri,
    Form(f): Form<FormJson>,
) -> impl IntoResponse {

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Ok(None) | Err(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(Some(err)) => {
            return Err(
                    add_msg(
                        err.to_string(),
                        "danger".to_string(),
                        "/account/login".to_string(),
                ).await
            )
        }
    };

    let _ = creat_cmt(
        i.pool.clone(), to_id, t.id, t.email, t.username, f, "users"
    ).await.unwrap();

    Ok(Redirect::to(original_uri.path()).into_response())
}

pub async fn get_update_cmt(
    headers: HeaderMap,
    Path((to_id, cid)): Path<(i32, i32)>,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            context.insert("err", &err);
            return Err(
                Html(templates.render("update_cmt", &context).unwrap())
            )
        }
        Ok(None) | Err(None) => {
            context.insert("err", "Caramba bullfighting and damn it");
            return Err(
                Html(templates.render("update_cmt", &context).unwrap())
            )
        }
    };
    let cmt = id_cmt(
        i.pool.clone(), to_id, t.id, cid, "users"
    ).await;
    match cmt {
        Ok(expr) => {
            context.insert("t", &t);
            context.insert("i", &expr);
            Ok(Html(templates.render("update_cmt", &context).unwrap()))
        }
        Err(Some(err)) => {
            context.insert("err", &err.to_string());
            Err(Html(templates.render("update_cmt", &context).unwrap()))
        }
        Err(None) => {
            context.insert("is_no", "Caramba bullfighting and damn it");
            Err(Html(templates.render("update_cmt", &context).unwrap()))
        }
    }
}

pub async fn post_update_cmt(
    headers: HeaderMap,
    Path((to_id, cid)): Path<(i32, i32)>,
    State(i): State<Arc<AuthRedis>>,
    Form(f): Form<FormUpdateCmt>,
) -> impl IntoResponse {

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Ok(None) | Err(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(Some(err)) => {
            return Err(
                    add_msg(
                        err.to_string(),
                        "danger".to_string(),
                        "/account/login".to_string(),
                ).await
            )
        }
    };

    let result = update_cmt(i.pool.clone(), to_id, t.id, cid, f, "users").await;
    match result {
        Ok(expr) => expr,
        Err(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(Some(err)) => {
            return Err(
                    add_msg(
                        "insert comment..! ".to_string() + &err.to_string(),
                        "danger".to_string(),
                        "/account/login".to_string(),
                ).await
            )
        }
    };

    Ok(
        Redirect::to(&("/creat-cmtjson/".to_owned() + &to_id.to_string()))
        .into_response()
    )
}

pub async fn get_cmt_del(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    match i.ctx(headers).await {
        Ok(Some(expr)) => {
            context.insert("t", &expr);
            Ok(Html(templates.render("cmt_del", &context).unwrap()))
        }
        Err(Some(err)) => {
            context.insert("err", &err);
            Err(
                Html(templates.render("cmt_del", &context).unwrap())
            )
        }
        Ok(None) | Err(None) => {
            context.insert("err", "Caramba bullfighting and damn it");
            Err(
                Html(templates.render("cmt_del", &context).unwrap())
            )
        }
    }
}

pub async fn post_cmt_del(
    headers: HeaderMap,
    Path((to_id, cid)): Path<(i32, i32)>,
    State(i): State<Arc<AuthRedis>>,
) -> impl IntoResponse {

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Ok(None) | Err(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(Some(err)) => {
            return Err(
                    add_msg(
                        err.to_string(),
                        "danger".to_string(),
                        "/account/login".to_string(),
                ).await
            )
        }
    };

    let result = del_cmt(i.pool.clone(), cid, t.id, to_id, "users").await;
    match result {
        Ok(expr) => expr,
        Err(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(Some(err)) => {
            return Err(
                    add_msg(
                        "insert comment..! ".to_string() + &err.to_string(),
                        "danger".to_string(),
                        "/account/login".to_string(),
                ).await
            )
        }
    };

    Ok(
        Redirect::to(&("/creat-cmtjson/".to_owned() + &to_id.to_string()))
        .into_response()
    )
}