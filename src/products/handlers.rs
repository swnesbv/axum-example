use std::sync::Arc;
use axum::{
    extract::{State, Path},
    response::{Html, IntoResponse, Redirect},
    Extension,
};
use tera::Context;

use crate::{
    common::Templates,
    auth::models::{AuthRedis},
    provision::models::ParsePointError,
    products::models::{FormSelect},
    products::views::{all_products, id_products, form_on_off, i_categories, i_cts},
};


pub async fn get_all(
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> impl IntoResponse {

    let all = all_products(i.pool.clone()).await.unwrap();

    let mut context = Context::new();
    context.insert("all", &all);
    Html(templates.render("all", &context).unwrap())
}

pub async fn get_detail(
    Path(id): Path<String>,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let number = match id.parse::<i32>().map_err(|_| ParsePointError) {
        Ok(expr) => expr,
        Err(err) => {
            context.insert("err", &err);
            return Err(Html(templates.render("detail", &context).unwrap()))
        }
    };
    let i = id_products(i.pool.clone(), number).await.unwrap();

    context.insert("i", &i);
    Ok(Html(templates.render("detail", &context).unwrap()))
}


pub async fn get_select(
    Extension(templates): Extension<Templates>,
) -> impl IntoResponse {
    Html(templates.render("select", &Context::new()).unwrap())
}

pub async fn post_select(
    axum_extra::extract::Form(f): axum_extra::extract::Form<FormSelect>,
) -> impl IntoResponse {

    let i = form_on_off(f).await;
    let s = serde_json::to_string(&i).unwrap();

    Redirect::to(&("/products/categories/".to_owned() + &s)).into_response()
}


pub async fn get_categories(
    Path(p): Path<String>,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> impl IntoResponse {

    let a: serde_json::Value = serde_json::from_str(&p).unwrap();

    let all = i_categories(
        i.pool.clone(), a[0].as_str(), a[1].as_str(), a[2].as_str(), a[3].as_str()
    ).await.unwrap();

    let mut context = Context::new();
    context.insert("all", &all);
    Html(templates.render("categories", &context).unwrap())
}


pub async fn get_cts(
    Path(p): Path<String>,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> impl IntoResponse {

    let a: Vec<&str> = p.split(",").collect();
    let mut b = Vec::<String>::new();
    for c in &a {
        b.push(c.to_string());
    }
    let all = i_cts(
        i.pool.clone(), Some(b)
        // pool, Some(vec!["c1".to_string(),"c2".to_string()])
    ).await.unwrap();

    let mut context = Context::new();
    context.insert("all", &all);
    Html(templates.render("cts", &context).unwrap())
}