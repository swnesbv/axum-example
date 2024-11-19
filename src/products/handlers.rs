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
    products::views::{all_products, id_products, form_on_off, i_categories, i_cts},
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

pub async fn get_detail(
    Path(id): Path<String>,
    State(pool): State<PgPool>,
    Extension(templates): Extension<Templates>,
) -> impl IntoResponse {

    let number: i32 = id.parse().expect("Not a valid number");
    let i = id_products(pool, number).await.unwrap();

    let mut context = Context::new();
    context.insert("i", &i);
    Html(templates.render("detail", &context).unwrap())
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

    let all = i_categories(
        pool, a[0].as_str(), a[1].as_str(), a[2].as_str(), a[3].as_str()
    ).await.unwrap();

    let mut context = Context::new();
    context.insert("all", &all);
    Html(templates.render("categories", &context).unwrap())
}


pub async fn get_cts(
    Path(i): Path<String>,
    State(pool): State<PgPool>,
    Extension(templates): Extension<Templates>,
) -> impl IntoResponse {

    let a: Vec<&str> = i.split(",").collect();
    let mut b = Vec::<String>::new();
    for i in &a {
        b.push(i.to_string());
    }
    let all = i_cts(
        pool, Some(b)
        // pool, Some(vec!["c1".to_string(),"c2".to_string()])
    ).await.unwrap();

    let mut context = Context::new();
    context.insert("all", &all);
    Html(templates.render("cts", &context).unwrap())
}