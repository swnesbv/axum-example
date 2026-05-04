use std::sync::Arc;
use axum::{
    extract::{Form, State, OriginalUri},
    response::{IntoResponse, Redirect},
    http::header::{HeaderMap}
};

use crate::{
    auth::models::AuthRedis,
    comments::models::{FormComment},
    comments::views::{insert_comment},
    photo::views::{add_msg}
};

pub async fn post_creat_cmt_user(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    OriginalUri(original_uri): OriginalUri,
    Form(f): Form<FormComment>,
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

    let result = insert_comment(
        i.pool.clone(), t.id, t.email, t.username, f, "users"
    ).await;
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

    Ok(Redirect::to(original_uri.path()).into_response())
}


/*pub async fn post_creat_cmt_provision(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    OriginalUri(original_uri): OriginalUri,
    Form(f): Form<FormComment>,
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

    let result = insert_cmt(
        f, i.pool.clone(), t.id, t.email, t.username, "provision_d"
    ).await;
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

    Ok(Redirect::to(original_uri.path()).into_response())
}*/