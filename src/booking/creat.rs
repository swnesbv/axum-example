// use chrono::{Utc};
use axum::{
    body::Body,
    extract::{
        // State,
        Form,
    },
    http::{Response, StatusCode},
    response::{
        // Redirect,
        Html,
        IntoResponse,
    },
    Extension,
};

use tera::Context;

// use headers::Cookie;
// use axum_extra::TypedHeader;

use crate::{
    booking::models::{Claims, FormSE},
    common::{
        // Pool,
        Templates,
    },
};

pub async fn get_period(Extension(templates): Extension<Templates>) -> impl IntoResponse {
    Html(templates.render("period", &Context::new()).unwrap())
}

pub async fn post_period(Form(form): Form<FormSE>) -> impl IntoResponse {
    let s_value = form.start;
    let e_value = form.end;

    let entity = Claims {
        start: s_value,
        end: e_value,
    };
    use bincode;
    let encoded: Vec<u8> = bincode::serialize(&entity).unwrap();

    use base64::{engine::general_purpose::STANDARD, Engine as _};
    let token = STANDARD.encode(encoded);

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
