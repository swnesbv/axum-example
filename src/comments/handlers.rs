use std::sync::Arc;
use axum::{
    extract::{Path, Form, State},
    response::{Html, IntoResponse, Redirect},
    http::header::{HeaderMap},
    Extension
};
use tera::Context;

use crate::{
    common::Templates,
    auth::models::AuthRedis,
    comments::models::{FormUpdateCmt},
    comments::views::{id_cmt},
    comments::repository::{replace_cms, cmt_del},
    photo::repository::{add_msg}
};

pub async fn get_update_cmt(
    headers: HeaderMap,
    Path((name, cid)): Path<(String, String)>,
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
    let cmt = id_cmt(i.pool.clone(), &name, &cid, "users").await;
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
    Path((name, cid)): Path<(String, String)>,
    State(i): State<Arc<AuthRedis>>,
) -> impl IntoResponse {

    let _ = match i.ctx(headers).await {
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

    let _ = cmt_del(i.pool.clone(), &name, &cid, "users").await.unwrap();
    // match result {
    //     Ok(expr) => expr,
    //     Err(None) => return Err(Redirect::to("/account/login").into_response()),
    //     Err(Some(err)) => {
    //         return Err(
    //                 add_msg(
    //                     "insert comment..! ".to_string() + &err.to_string(),
    //                     "danger".to_string(),
    //                     "/account/login".to_string(),
    //             ).await
    //         )
    //     }
    // };

    Ok(
        Redirect::to(&("/account/user/".to_owned() + &name))
        .into_response()
    )
}

pub async fn post_update_cmt(
    headers: HeaderMap,
    Path((name, cid)): Path<(String, String)>,
    State(i): State<Arc<AuthRedis>>,
    Form(f): Form<FormUpdateCmt>,
) -> impl IntoResponse {

    let _ = match i.ctx(headers).await {
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

    let _ = replace_cms(i.pool.clone(), &name, &cid, f, "users").await.unwrap();
    // match result {
    //     Ok(expr) => expr,
    //     Err(None) => return Err(Redirect::to("/account/login").into_response()),
    //     Err(Some(err)) => {
    //         return Err(
    //                 add_msg(
    //                     "insert comment..! ".to_string() + &err.to_string(),
    //                     "danger".to_string(),
    //                     "/account/login".to_string(),
    //             ).await
    //         )
    //     }
    // };

    Ok(
        Redirect::to(&("/account/user/".to_owned() + &name))
        .into_response()
    )
}
