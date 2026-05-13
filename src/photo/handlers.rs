use std::sync::Arc;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect},
    http::header::{HeaderMap},
    Extension,
};
use tera::Context;

use crate::{
    common::Templates,
    auth::models::AuthRedis,
    photo::models::FormUpdateSlider,
    photo::repository::{slider_id, sl_photo, zip_collection}
};

pub async fn get_update_slider(
    headers: HeaderMap,
    Path((id, _)): Path<(i32, i32)>,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let mut context = Context::new();
    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            context.insert("err", &err.to_string());
            return Ok(Html(templates.render("update_slider", &context).unwrap()));
        }
        Ok(None) | Err(None) => return Err(Redirect::to("/account/login").into_response()),
    };

    let slider = slider_id(i.pool.clone(), id, t.id).await.unwrap();
    let collection = zip_collection(i.pool.clone(), t.id).await.unwrap();

    context.insert("t", &t);
    context.insert("i", &slider);
    context.insert("collection", &collection);
    Ok(Html(templates.render("update_slider", &context).unwrap()))
}

pub async fn post_update_slider(
    headers: HeaderMap,
    Path((id, to_product)): Path<(i32, i32)>,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
    axum_extra::extract::Form(f): axum_extra::extract::Form<FormUpdateSlider>,
) -> impl IntoResponse {

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            let mut context = Context::new();
            context.insert("err", &err.to_string());
            return Err(
                Html(templates.render("update_slider", &context).unwrap())
            );
        }
        Ok(None) | Err(None) => {
            let mut context = Context::new();
            context.insert("is_no", "Caramba bullfighting and damn it");
            return Err(
                Html(templates.render("update_slider", &context).unwrap())
            )
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
    //..
    let pg = match i.pool.get().await{
        Ok(expr) => expr,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err", &err.to_string());
            return Err(
                Html(templates.render("update_slider", &context).unwrap())
            );
        }
    };
    let result = pg.execute(
        "UPDATE slider SET title=$3, description=$4, img=$5, updated_at=now() WHERE id=$1 AND user_id=$2", &[&id, &t.id, &vec_t, &vec_d, &vec_p]
    ).await;
    match result {
        Ok(expr) => expr,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err", &err.to_string());
            return Err(
                Html(templates.render("update_slider", &context).unwrap())
            );
        }
    };
    Ok(
        Redirect::to(
            &("/products/detail/".to_owned() + &to_product.to_string())
        ).into_response()
    )
}


pub async fn get_collections(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();
    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            context.insert("err", &err.to_string());
            return Err(
                Html(templates.render("collections", &context).unwrap())
            );
        }
        Ok(None) | Err(None) => {
            context.insert("is_no", "Caramba bullfighting and damn it");
            return Err(
                Html(templates.render("collections", &context).unwrap())
            )
        }
    };
    let all = zip_collection(i.pool.clone(), t.id).await.unwrap();

    context.insert("all", &all);
    context.insert("t", &t);
    Ok(Html(templates.render("collections", &context).unwrap()))
}


pub async fn get_slider_photo(
    Path(id): Path<i32>,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let user = sl_photo(i.pool.clone(), id).await;
    match user {
        Ok(expr) => {
            context.insert("i", &expr);
            Ok(Html(templates.render("slider_photo", &context).unwrap()))
        }
        Err(Some(err)) => {
            context.insert("err", &err.to_string());
            Err(Html(templates.render("slider_photo", &context).unwrap()))
        }
        Err(None) => {
            context.insert("is_no", "Caramba bullfighting and damn it");
            Err(Html(templates.render("slider_photo", &context).unwrap()))
        }
    }
}


