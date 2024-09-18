use sqlx::postgres::PgPool;

use axum::{
    extract::{State},
    response::{Html, IntoResponse, Redirect},
    Extension,
};

use chrono::{Utc};

use tera::Context;

use axum_extra::TypedHeader;
use headers::Cookie;

use crate::{
    auth,
    common::Templates,
    products::models::{FormProducts, AmountPrice}
};


pub async fn get_creat(
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let token = auth::views::request_user(cookie).await;
    let _ = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(_) => return Err(Redirect::to("/account/login").into_response()),
    };
    Ok(Html(templates.render("creat", &Context::new()).unwrap()))
}


pub async fn post_creat(
    State(pool): State<PgPool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
    axum_extra::extract::Form(form): axum_extra::extract::Form<FormProducts>,
) -> impl IntoResponse {

    let token = auth::views::request_token(cookie).await.unwrap();

    let on_off = form.on_off;
    let categories = form.categories;

    let mut f: Vec<String> = vec![];
    let mut e = vec![];

    for i in on_off {
        let g = i.parse::<String>().unwrap();
        f.push(g);
    }
    for (c, d) in f.iter().zip(categories.iter()) {
        if *c == "1" {
            e.push(d.to_owned());
        }
    }

    let a_container = form.a_container;
    let a_boxes = form.a_boxes;
    let a_carton = form.a_carton;
    let a_units = form.a_units;

    let p_container = form.p_container;
    let p_boxes = form.p_boxes;
    let p_carton = form.p_carton;
    let p_units = form.p_units;

    let a: AmountPrice = AmountPrice {
        container: a_container,
        boxes: a_boxes,
        carton: a_carton,
        units: a_units
    };
    let str_a = serde_json::to_string(&a).unwrap();
    let amount: serde_json::Value = serde_json::from_str(&str_a).unwrap();

    let p: AmountPrice = AmountPrice {
        container: p_container,
        boxes: p_boxes,
        carton: p_carton,
        units: p_units
    };
    let str_p = serde_json::to_string(&p).unwrap();
    let price: serde_json::Value = serde_json::from_str(&str_p).unwrap();

    let result = sqlx::query(
        "INSERT INTO products (user_id, title, description, categories, amount, price, created_at) VALUES ($1,$2,$3,$4,$5,$6,$7)"
        )
        .bind(token.claims.id)
        .bind(&form.title)
        .bind(&form.description)
        .bind(e)
        .bind(amount)
        .bind(price)
        .bind(Utc::now())
        .execute(&pool)
        .await;
    match result {
        Ok(result) => result,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err_token", &err.to_string());
            return Err(Html(templates.render("creat", &context).unwrap()));
        }
    };
    Ok(Redirect::to("/products/all").into_response())
}
