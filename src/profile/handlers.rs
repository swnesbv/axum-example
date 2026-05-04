use std::sync::Arc;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
    http::{header::{HeaderMap}},
    Extension
};
// use redis::{AsyncCommands, RedisError};
use tera::Context;

use crate::{
    auth::models::{AuthRedis},
    auth::check::{in_check},
    common::{Templates},
    profile::views::{all, details},
    comments::views::{i_comments},
    subscriptions::repository::{check_ssc},
};


// #[axum::debug_handler()]
pub async fn index(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    // let json_key: Result<Option<String>, RedisError> = rs.get("session").await;
    // let key: Result<Option<String>, RedisError> = rs.get("key").await;
    // let email: Result<Option<String>, RedisError> = rs.get("email").await;
    // match key {
    //     Ok(_) => match email {
    //         Ok(ref expr) => expr,
    //         Err(err) => {
    //             context.insert("err", &err.to_string());
    //             return Err(
    //                 Html(templates.render("index", &context).unwrap())
    //             )
    //         }
    //     },
    //     Err(err) => {
    //         context.insert("err", &err.to_string());
    //         return Err(Html(templates.render("index", &context).unwrap()))
    //     }
    // };

    let _ = all(i.pool.clone()).await.unwrap();
    //..

    let a = i.ctx(headers).await;

    match a {
        Ok(expr) => {
            context.insert("t", &expr);
            Ok(Html(templates.render("index", &context).unwrap()))
        },
        Err(Some(err)) => {
            context.insert("err", &err.to_string());
            Err(Html(templates.render("index", &context).unwrap()))
        }
        Err(None) => {
            context.insert("is_no", "index Err Caramba bullfighting and damn it..!");
            Err(Html(templates.render("index", &context).unwrap()))
        }
    }

}

pub async fn users(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let _ = match in_check(i.conn.clone(), headers.clone()).await {
        Ok(expr) => {
            let _ = match check_ssc(i.pool.clone()).await {
                Ok(ssc) => {
                    context.insert("ssc", &ssc);
                    Ok(Html(templates.render("users", &context).unwrap()))
                }
                Err(Some(err)) => {
                    context.insert("err", &err);
                    return Err(Html(templates.render("users", &context).unwrap()))
                }
                Err(None) => {
                    context.insert("is_no", "is not subscription..!");
                    Err(Html(templates.render("users", &context).unwrap()))
                }
            };
            context.insert("visit", &expr);
            Ok(Html(templates.render("users", &context).unwrap()))
        }
        Err(Some(err)) => {
            context.insert("err", &err);
            Err(Html(templates.render("users", &context).unwrap()))
        }
        Err(None) => {
            context.insert("is_no", "Err-None Caramba bullfighting and damn it");
            Err(Html(templates.render("users", &context).unwrap()))
        }
    };
    match all(i.pool.clone()).await {
        Ok(expr) => {
            context.insert("all_users", &expr);
            Ok(Html(templates.render("users", &context).unwrap()))
        }
        Err(Some(err)) => {
            context.insert("err", &err.to_string());
            Err(Html(templates.render("users", &context).unwrap()))
        }
        Err(None) => {
            context.insert("is_no", "Err-None Caramba bullfighting and damn it");
            Err(Html(templates.render("users", &context).unwrap()))
        }
    }
}

pub async fn user(
    Path(name): Path<String>,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let user = details(i.pool.clone(), name.clone()).await;
    let _ = match user {
        Ok(expr) => {
            context.insert("i", &expr);
            Ok(Html(templates.render("user", &context).unwrap()))
        }
        Err(Some(err)) => {
            context.insert("err", &err.to_string());
            Err(Html(templates.render("user", &context).unwrap()))
        }
        Err(None) => {
            context.insert("is_no", "Caramba bullfighting and damn it");
            Err(Html(templates.render("user", &context).unwrap()))
        }
    };
    let cmt = i_comments(i.pool.clone(), &name).await;
    match cmt {
        Ok(expr) => {
            context.insert("cmt", &expr);
            Ok(Html(templates.render("user", &context).unwrap()))
        }
        Err(Some(err)) => {
            context.insert("err", &err.to_string());
            Err(Html(templates.render("user", &context).unwrap()))
        }
        Err(None) => {
            context.insert("is_no", "Caramba bullfighting and damn it");
            Err(Html(templates.render("user", &context).unwrap()))
        }
    }

}
