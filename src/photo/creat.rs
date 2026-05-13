use std::sync::Arc;
// use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use axum::{
    extract::{Multipart, State},
    response::{Html, IntoResponse, Redirect},
    http::header::{HeaderMap},
    Extension,
};
use chrono::{Utc};
use tera::Context;
use zip::ZipArchive;

use crate::{
    common::Templates,
    auth::models::AuthRedis,
    photo::views::{insert_collection},
    photo::repository::{add_msg, zip_collection},
    photo::models::{FormSlider},
    products::views::{user_products},
};

pub async fn get_creat_slider(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let mut context = Context::new();
    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            context.insert("err", &err.to_string());
            return Ok(Html(templates.render("creat", &context).unwrap()));
        }
        Ok(None) | Err(None) => return Err(Redirect::to("/account/login").into_response()),
    };

    let products = user_products(i.pool.clone(), t.id).await.unwrap();
    let collection = zip_collection(i.pool.clone(), t.id).await.unwrap();

    context.insert("t", &t);
    context.insert("products", &products);
    context.insert("collection", &collection);
    Ok(Html(templates.render("creat", &context).unwrap()))
}

pub async fn post_creat_slider(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
    axum_extra::extract::Form(f): axum_extra::extract::Form<FormSlider>,
) -> impl IntoResponse {

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            let mut context = Context::new();
            context.insert("err", &err.to_string());
            return Err(Html(templates.render("creat", &context).unwrap()));
        }
        Ok(None) | Err(None) => {
            let mut context = Context::new();
            context.insert("is_no", "Caramba bullfighting and damn it");
            return Err(Html(templates.render("creat", &context).unwrap()))
        }
    };

    let mut v: Vec<String> = vec![];
    let mut path           = vec![];
    let mut title          = vec![];
    let mut description    = vec![];

    let on_off = f.on_off;
    for x in on_off {
        let y = x.parse::<String>().unwrap();
        v.push(y);
    }
    let img = f.img;
    for (a, b) in v.iter().zip(img.iter()) {
        if *a == "1" {
            path.push(b.to_owned());
        }
    }
    let vec_p = serde_json::to_value(&path).unwrap();
    //..
    for a in f.title {
       title.push(a);
    }
    let vec_t = serde_json::to_value(title).unwrap();
    for b in f.description {
       description.push(b);
    }
    let vec_d = serde_json::to_value(description).unwrap();

    let pg = match i.pool.get().await{
        Ok(expr) => expr,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err", &err.to_string());
            return Err(Html(templates.render("creat", &context).unwrap()));
        }
    };
    let result = pg.execute(
        "INSERT INTO slider (user_id, to_product, title, description, img, created_at) VALUES ($1,$2,$3,$4,$5,now())",
        &[&t.id, &f.to_product, &vec_t, &vec_d, &vec_p]
    ).await;
    match result {
        Ok(expr) => expr,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err", &err.to_string());
            return Err(Html(templates.render("creat", &context).unwrap()));
        }
    };
    Ok(
        Redirect::to(&("/products/detail/".to_owned() + &f.to_product.to_string())).into_response()
    )
}

pub async fn post_collections_zip(
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

    let mut dir  = String::from("");
    let mut path = String::from("");
    let mut vec_img: Vec<String> = vec![];

    while let Some(f) = multipart.next_field().await.unwrap() {

        let f_zip = f.file_name().unwrap().to_string();
        let data = f.bytes().await.unwrap();

        dir = "./static/assets/photo/zip/".to_owned() + &t.email + "/";
        if fs::exists(&dir).unwrap() {
            fs::remove_dir_all(&dir).unwrap();
        }
        let _ = fs::create_dir_all(&dir);
        path = dir.clone() + &f_zip;
        let mut buffer = File::create(&path).unwrap();
        buffer.write_all(&data).unwrap();
    }

    let zip_file = File::open(&path).unwrap();
    let mut archive = ZipArchive::new(zip_file).unwrap();

    let w = "./static/assets/photo/slider/".to_owned() + &t.email + "/";
    let way = std::path::Path::new(&w);

    if fs::exists(way).unwrap() {
        fs::remove_dir_all(way).unwrap();
    }
    let _ = fs::create_dir_all(way);
    let mut new = 0;

    let j = "/assets/photo/slider/".to_owned() + &t.email + "/";
    let wj = std::path::Path::new(&j);

    for x in 0..archive.len() {
        let mut f = archive.by_index(x).unwrap();
        //..
        let enclosed_name = f.enclosed_name().unwrap();
        let file_name = enclosed_name.file_name();
        let v: Vec<&str> = file_name.expect("").to_str().expect("").split(".").collect();
        new += 1;
        let name_new = format!("{}.{}", new, v[1]);
        if new == 11{
            break;
        }
        //..
        let utc      = Utc::now().format("%d%m%Y%H%M%S");
        let name     = format!("{}/{}", utc, name_new);
        let way_name = way.join(&name);
        let s = wj.join(&name).display().to_string();
        vec_img.push(s);

        if let Some(parent_dir) = way_name.parent() {
            fs::create_dir_all(parent_dir).unwrap();
        }

        let mut output_file = File::create(&way_name).unwrap();
        io::copy(& mut f, & mut output_file).unwrap();
    }
    fs::remove_dir_all(&dir).unwrap();

    let json_img: serde_json::Value = serde_json::to_value(&vec_img).unwrap();
    insert_collection(i.pool.clone(), t.id, 1, json_img).await.unwrap();

    Ok(Redirect::to("/collections").into_response())
}


pub async fn get_photo(
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

pub async fn post_photo_user(
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

