use axum::{
    body::Body,
    extract::{Form},
    http::{Response, StatusCode},
    response::{Html, IntoResponse},
    Extension,
};

use tera::Context;

use crate::{
    booking::models::{FormSE},
    common::{
        Templates,
    },
};


pub async fn get_period(Extension(templates): Extension<Templates>) -> impl IntoResponse {
    Html(templates.render("period", &Context::new()).unwrap())
}

pub async fn post_period(
    Form(form): Form<FormSE>
) -> impl IntoResponse {

    let s_value = form.start;
    let e_value = form.end;

    let token = s_value + "," + &e_value;

    Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", "/booking/search-period-days")
        .header(
            "Set-Cookie",
            format!(
                "{}={}; Path={}; HttpOnly={}; SameSite={}",
                "period", token, "/", "true", "lax",
            ),
        )
        .body(Body::from("not found"))
        .unwrap()
}
