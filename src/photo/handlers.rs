use axum::{
    extract::{Multipart, State},
    response::{Html, IntoResponse, Redirect},
    Extension,
};
use chrono::Utc;

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use std::fs;
use std::fs::File;
use std::io::prelude::*;

use tera::Context;

use axum_extra::TypedHeader;
use headers::Cookie;

use crate::{auth, schema};
use crate::{
    common::{Pool, Templates},
    photo::models::ImgUser,
};

pub use axum_macros::debug_handler;

#[debug_handler]
pub async fn get_photo_users(Extension(templates): Extension<Templates>) -> impl IntoResponse {
    Html(templates.render("photo", &Context::new()).unwrap())
}

#[debug_handler]
pub async fn photo_users(
    State(pool): State<Pool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let token = auth::views::request_user(TypedHeader(cookie)).await;
    let ok_token = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(_) => return Err(Redirect::to("/account/login").into_response()),
    };

    let mut conn = pool.get().await.unwrap();
    use schema::users::dsl::*;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let f_name = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        let _ = fs::create_dir_all("./assets/img/user");
        let mut buffer = File::create(format!("./assets/img/user/{}", f_name)).unwrap();
        buffer.write_all(&data).unwrap();

        let img_user = ImgUser {
            img: format!("/assets/img/user/{}", f_name),
            updated_at: Utc::now(),
        };

        let _ = diesel::update(users.filter(id.eq(ok_token.id)))
            .set(img_user)
            .execute(&mut conn)
            .await;
    }
    Ok(Redirect::to("/").into_response())
}
