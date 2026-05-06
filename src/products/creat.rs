use std::sync::Arc;
use axum::{
    extract::{State},
    response::{Html, IntoResponse, Redirect},
    http::{header::{HeaderMap}},
    Extension,
};
use tera::Context;

use crate::{
    common::Templates,
    auth::models::{AuthRedis},
    products::models::{FormProducts, AmountPrice}
};

pub async fn get_creat(
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

    context.insert("t", &t);
    Ok(Html(templates.render("creat", &context).unwrap()))
}


pub async fn post_creat(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
    axum_extra::extract::Form(f): axum_extra::extract::Form<FormProducts>,
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
    let mut e = vec![];

    let on_off = f.on_off;
    for x in on_off {
        let y = x.parse::<String>().unwrap();
        v.push(y);
    }
    let categories = f.categories;
    for (c, d) in v.iter().zip(categories.iter()) {
        if *c == "1" {
            e.push(d.to_owned());
        }
    }

    let a_container = f.a_container;
    let a_boxes     = f.a_boxes;
    let a_carton    = f.a_carton;
    let a_units     = f.a_units;

    let p_container = f.p_container;
    let p_boxes     = f.p_boxes;
    let p_carton    = f.p_carton;
    let p_units     = f.p_units;

    let a: AmountPrice = AmountPrice {
        container: a_container,
        boxes:     a_boxes,
        carton:    a_carton,
        units:     a_units
    };
    let str_a = serde_json::to_string(&a).unwrap();
    let amount: serde_json::Value = serde_json::from_str(&str_a).unwrap();

    let p: AmountPrice = AmountPrice {
        container: p_container,
        boxes:     p_boxes,
        carton:    p_carton,
        units:     p_units
    };
    let str_p = serde_json::to_string(&p).unwrap();
    let price: serde_json::Value = serde_json::from_str(&str_p).unwrap();

    let pg = match i.pool.get().await{
        Ok(expr) => expr,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err", &err.to_string());
            return Err(Html(templates.render("creat", &context).unwrap()));
        }
    };
    let result = pg.execute(
        "INSERT INTO products (user_id, title, description, categories, cts, amount, price, created_at) VALUES ($1,$2,$3,$4,$5,$6,$7,now())", &[&t.id, &f.title, &f.description, &e, &f.cts.as_deref(), &amount, &price]
    ).await;
    match result {
        Ok(expr) => expr,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err", &err.to_string());
            return Err(Html(templates.render("creat", &context).unwrap()));
        }
    };
    Ok(Redirect::to("/products/all").into_response())
}
