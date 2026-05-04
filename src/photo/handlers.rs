use std::sync::Arc;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use axum::{
    extract::{Multipart, State},
    response::{Html, IntoResponse, Redirect},
    http::header::{HeaderMap},
    Extension,
};
use tera::Context;

use crate::{
    common::Templates,
    auth::models::AuthRedis,
    photo::views::add_msg
};


pub async fn get_photo_users(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> impl IntoResponse {
    let mut context = Context::new();

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Ok(None) | Err(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(Some(err)) => {
            return Err(
                add_msg(
                    err,
                    "/account/login".to_string(),
                    "danger".to_string()
            )
            .await)
        }
    };
    context.insert("t", &t);
    Ok(Html(templates.render("photo", &context).unwrap()))
}

pub async fn photo_users(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut context = Context::new();

    let t = match i.ctx(headers.clone()).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            context.insert("err", &err);
            return Err(Html(templates.render("photo", &context).unwrap()))
        }
        Ok(None) | Err(None) => {
            context.insert("is_no", "Caramba bullfighting and damn it");
            return Err(Html(templates.render("photo", &context).unwrap()))
        }
    };

    while let Some(f) = multipart.next_field().await.unwrap() {

        let path = "./static/assets/photo/user/".to_owned() + &t.email;
        if fs::exists(&path).unwrap() {
            fs::remove_dir_all(&path).unwrap();
        }
        let _ = fs::create_dir_all(&path);

        let f_name = f.file_name().unwrap().to_string();
        let v: Vec<&str> = f_name.split(".").collect();
        let utc = chrono::Utc::now().format("%d-%m-%Y_%H:%M:%S");
        let name_new = format!("{} {}", utc, v[1]);
        println!(" name_new..! {:?}", name_new);

        let creat_path = format!(
            "./static/assets/photo/user/{}/{}", t.email, f_name
        );
        let mut buffer = File::create(&creat_path).unwrap();
        let data = f.bytes().await.unwrap();
        buffer.write_all(&data).unwrap();

        let pg = match i.pool.get().await{
            Ok(expr) => expr,
            Err(err) => {
                context.insert("err", &err.to_string());
                return Err(
                    Html(templates.render("photo", &context).unwrap())
                )
            }
        };
        let save_path = format!(
            "/assets/photo/user/{}/{}", t.email, f_name
        );
        let result = pg.execute(
            "UPDATE users SET img=$2, updated_at=now() WHERE id=$1",
            &[&t.id, &save_path]
        ).await;
        let _ = match result {
            Ok(result) => Ok(result),
            Err(err) => Err(err.to_string()),
        };
    }
    Ok(Redirect::to(&("/account/user/".to_owned() + &t.username.to_string())).into_response())
}
