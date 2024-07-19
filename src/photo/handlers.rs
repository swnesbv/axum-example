use sqlx::postgres::PgPool;

use axum::{
    extract::{Multipart, State},
    response::{Html, IntoResponse, Redirect},
    Extension,
};
use chrono::Utc;

use std::fs;
use std::fs::File;
use std::io::prelude::*;

use tera::Context;

use axum_extra::TypedHeader;
use headers::Cookie;

use crate::{
    auth,
    common::{Templates},
    photo::models::ImgUser,
    photo::views::{add_msg},
};


pub async fn get_photo_users(
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
) -> impl IntoResponse {

    let token = auth::views::request_user(TypedHeader(cookie)).await;
    match token {
        Ok(Some(token)) => token,
        Ok(None) =>  return Err(Redirect::to("/account/login").into_response()),
        Err(err) => return Err(
            add_msg(
                err.expect("REASON").to_string(), "/account/login".to_string(), "danger".to_string()
            ).await
        )
    };
    Ok(Html(templates.render("photo", &Context::new()).unwrap()))
}


pub async fn photo_users(
    State(pool): State<PgPool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
    mut multipart: Multipart,
) -> impl IntoResponse {

    let mut context = Context::new();

    let token = auth::views::request_user(TypedHeader(cookie)).await;
    let t = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Ok(Redirect::to("/account/login").into_response()),
        Err(err) => {
                context.insert("err_token", &err.expect("REASON").to_string());
                return Err(Html(templates.render("photo", &context).unwrap()));
            }
    };

    while let Some(field) = multipart.next_field().await.unwrap() {
        let f_name = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        let _ = fs::create_dir_all("./assets/img/user");
        let mut buffer = File::create(format!("./assets/img/user/{}", f_name)).unwrap();
        buffer.write_all(&data).unwrap();

        let i = ImgUser {
            img: format!("/assets/img/user/{}", f_name),
            updated_at: Utc::now(),
        };

        sqlx::query_as!(
            ImgUser, "UPDATE users SET img=$2, updated_at=$3 WHERE id=$1", t.id, &i.img, Some(Utc::now())
            )
            .fetch_one(&pool)
            .await
            .unwrap();
    }
    Ok(
        Redirect::to(&("/account/user/".to_owned() + &t.id.to_string())).into_response()
    )
}
