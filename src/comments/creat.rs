use axum::{
    extract::{Form, OriginalUri},
    response::{IntoResponse, Redirect}
};

use chrono::{Utc};

use axum_extra::TypedHeader;
use headers::Cookie;

use crate::{
    auth,
    common::DatabaseConn,
    comments::models::{FormComment, CommentOn}
};


pub async fn post_creat(
    DatabaseConn(mut conn): DatabaseConn,
    TypedHeader(cookie): TypedHeader<Cookie>,
    OriginalUri(original_uri): OriginalUri,
    Form(form): Form<FormComment>,
) -> impl IntoResponse {

    let token = auth::views::request_token(cookie).await.unwrap();

    let m: CommentOn = CommentOn {
        user_id: token.claims.id,
        email: token.claims.email,
        name: token.claims.username,
        whose: form.whose,
        msg: form.comment,
    };

    let str_msg = serde_json::to_string(&m).unwrap();
    let comment_on: serde_json::Value = serde_json::from_str(&str_msg).unwrap();

    let result = sqlx::query(
        "INSERT INTO comments (user_id, comment_on, created_at) VALUES ($1,$2,$3)"
        )
        .bind(token.claims.id)
        .bind(comment_on)
        .bind(Utc::now())
        .execute(&mut *conn)
        .await;
    match result {
        Ok(result) => result,
        Err(_) => {
            return Err(Redirect::to("/account/login").into_response());
        }
    };

    Ok(Redirect::to(original_uri.path()).into_response())
}
