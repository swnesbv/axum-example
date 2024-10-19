use sqlx::postgres::PgPool;

use axum::{
    body::Body,
    http::{StatusCode},
    extract::{State, Path, Form},
    response::{Html, IntoResponse, Redirect, Response},
    Extension,
};

// use chrono::{Utc};

use tera::Context;

use axum_extra::TypedHeader;
use headers::Cookie;

use crate::{
    auth,
    common::Templates,
    products::models::{AmountPrice},
    products::views::{id_products},
    purchases::models::{PurchasesCls, FormPurchases},
};


pub async fn get_order(
    Path(id): Path<String>,
    State(pool): State<PgPool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let token = auth::views::request_user(cookie).await;
    let _ = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(_) => return Err(Redirect::to("/account/login").into_response()),
    };

    let number: i32 = id.parse().expect("Not a valid number");
    let i = id_products(pool, number).await.unwrap();

    let mut context = Context::new();
    context.insert("i", &i);
    Ok(Html(templates.render("order", &context).unwrap()))
}


async fn convert(ia: Option<i32>, ib: Option<i32>) -> Option<i32> {
    let a: Option<i32> = ia;
    let b: Option<i32> = ib;
    let f = |a, b| {
        a * b
    };
    a.and_then(|a| b.map(|b| f(a, b)))
}

pub async fn post_order(
    State(pool): State<PgPool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Form(form): Form<FormPurchases>,
) -> impl IntoResponse {

    let token = auth::views::request_token(cookie).await.unwrap();

    let i = id_products(pool, form.product_id).await.unwrap();

    let a_container = form.a_container;
    let a_boxes = form.a_boxes;
    let a_carton = form.a_carton;
    let a_units = form.a_units;

    let a: AmountPrice = AmountPrice {
        container: a_container,
        boxes: a_boxes,
        carton: a_carton,
        units: a_units
    };
    let str_a = serde_json::to_string(&a).unwrap();
    let amount: serde_json::Value = serde_json::from_str(&str_a).unwrap();

    let str_i = serde_json::to_string(&i.price).unwrap();
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
        user_id:    token.claims.id,
        product_id: form.product_id,
        amount:     Some(amount),
        price:      Some(price),
    };

    println!(" entity..{:?}", entity);

    use bincode;
    let encoded: Vec<u8> = bincode::serialize(&entity).unwrap();
    use base64::{engine::general_purpose::STANDARD, Engine as _};
    let token = STANDARD.encode(encoded);

    Response::builder()
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
        .unwrap()

}
