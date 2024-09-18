use sqlx::postgres::PgPool;

use axum::{
    extract::{State, Path},
    response::{Html, IntoResponse, Redirect},
    Extension,
};

use tera::Context;

use crate::{
    common::Templates,
    products::models::{FormSelect},
    products::views::{all_products, form_on_off, i_categories},
};


pub async fn get_all(
    State(pool): State<PgPool>,
    Extension(templates): Extension<Templates>,
) -> impl IntoResponse {

    let all = all_products(pool).await.unwrap();

    let mut context = Context::new();
    context.insert("all", &all);
    Html(templates.render("all", &context).unwrap())
}


pub async fn get_select(
    Extension(templates): Extension<Templates>,
) -> impl IntoResponse {
    Html(templates.render("select", &Context::new()).unwrap())
}

pub async fn post_select(
    axum_extra::extract::Form(form): axum_extra::extract::Form<FormSelect>,
) -> impl IntoResponse {

    let i = form_on_off(form).await;
    let s = serde_json::to_string(&i).unwrap();

    Redirect::to(&("/products/categories/".to_owned() + &s)).into_response()
}


pub async fn get_categories(
    Path(i): Path<String>,
    State(pool): State<PgPool>,
    Extension(templates): Extension<Templates>,
) -> impl IntoResponse {

    let a: serde_json::Value = serde_json::from_str(&i).unwrap();

    println!(" a..{:?}", a[0]);

    let all = i_categories(
        pool, a[0].as_str(), a[1].as_str(), a[2].as_str(), a[3].as_str()
    ).await.unwrap();

    let mut context = Context::new();
    context.insert("all", &all);
    Html(templates.render("categories", &context).unwrap())
}