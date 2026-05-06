use std::sync::Arc;
use axum::{
    body::Body,
    http::{StatusCode},
    http::{header::{HeaderMap}},
    extract::{State, Path, Form},
    response::{Html, IntoResponse, Response},
    Extension,
};
// use chrono::{Utc};
use tera::Context;

use crate::{
    common::Templates,
    auth::models::{AuthRedis},
    products::models::{AmountPrice},
    products::views::{id_products},
    purchases::models::{PurchasesCls, FormPurchases},
    provision::models::{ParsePointError}
};


pub async fn get_order(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            context.insert("err", &err);
            return Err(Html(templates.render("order", &context).unwrap()))
        }
        Ok(None) | Err(None) => {
            context.insert("err", "Caramba bullfighting and damn it");
            return Err(Html(templates.render("order", &context).unwrap()))
        }
    };
    let number = match id.parse::<i32>().map_err(|_| ParsePointError) {
        Ok(expr) => expr,
        Err(err) => {
            context.insert("err", &err);
            return Ok(Html(templates.render("detail_days", &context).unwrap()))
        }
    };

    let i = id_products(i.pool.clone(), number).await.unwrap();

    context.insert("t", &t);
    context.insert("i", &i);
    Ok(Html(templates.render("order", &context).unwrap()))
}


async fn convert(
    ia: Option<i32>, ib: Option<i32>
) -> Option<i32> {
    let a: Option<i32> = ia;
    let b: Option<i32> = ib;
    let c = |a, b| {a * b};
    a.and_then(|a| b.map(|b| c(a, b)))
}

pub async fn post_order(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
    Form(f): Form<FormPurchases>,
) -> impl IntoResponse {

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            let mut context = Context::new();
            context.insert("err", &err);
            return Err(Html(templates.render("detail_days", &context).unwrap()))
        }
        Ok(None) | Err(None) => {
            let mut context = Context::new();
            context.insert("err", "Caramba bullfighting and damn it");
            return Err(Html(templates.render("detail_days", &context).unwrap()))
        }
    };

    let id = id_products(i.pool.clone(), f.product_id).await.unwrap();

    let a_container = f.a_container;
    let a_boxes     = f.a_boxes;
    let a_carton    = f.a_carton;
    let a_units     = f.a_units;

    let a: AmountPrice = AmountPrice {
        container: a_container,
        boxes:     a_boxes,
        carton:    a_carton,
        units:     a_units
    };
    let str_a = serde_json::to_string(&a).unwrap();
    let amount: serde_json::Value = serde_json::from_str(&str_a).unwrap();

    let str_i = serde_json::to_string(&id.price).unwrap();
    let kv: AmountPrice = serde_json::from_str(&str_i).unwrap();

    let p_container = convert(a.container, kv.container).await;
    let p_boxes     = convert(a.boxes, kv.boxes).await;
    let p_carton    = convert(a.carton, kv.carton).await;
    let p_units     = convert(a.units, kv.units).await;

    let p: AmountPrice = AmountPrice {
        container: p_container,
        boxes:     p_boxes,
        carton:    p_carton,
        units:     p_units
    };

    let str_p = serde_json::to_string(&p).unwrap();
    let price: serde_json::Value = serde_json::from_str(&str_p).unwrap();

    let entity = PurchasesCls {
        user_id:    t.id,
        product_id: f.product_id,
        amount:     Some(amount),
        price:      Some(price),
    };

    unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
        ::core::slice::from_raw_parts(
            (p as *const T) as *const u8,
            ::core::mem::size_of::<T>(),
        )
    }
    let encoded: &[u8] = unsafe { any_as_u8_slice(&entity) };
    use base64::{engine::general_purpose::STANDARD, Engine as _};
    let token = STANDARD.encode(encoded);

    Ok(Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", "/")
        .header(
            "Set-Cookie",
            format!(
                "{}={}; Path={}; HttpOnly={}; SameSite={}",
                "purchases", token, "/", "true", "lax",
            ),
        )
        .body(Body::from("not found"))
        .unwrap())

}
