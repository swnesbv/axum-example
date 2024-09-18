use sqlx::postgres::PgPool;

use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect},
    // http::{Request,Response,StatusCode},
    // body::Body,
    Extension,
};

use tera::Context;

use axum_extra::TypedHeader;
use headers::Cookie;
// use chrono::NaiveDate;

use crate::{
    booking::models::Claims,
    booking::views::{all, slt},
    common::Templates,
};


pub async fn bkg_all(
    State(pool): State<PgPool>,
    Extension(templates): Extension<Templates>,
) -> impl IntoResponse {
    let bkg = all(pool).await.unwrap();

    let mut context = Context::new();
    context.insert("bkg", &bkg);
    Html(templates.render("all_booking", &context).unwrap())
}


pub async fn search_days(
    State(pool): State<PgPool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let token = match cookie.get("period") {
        Some(expr) => expr,
        None => return Err(
            Redirect::to("/booking/creat-period-days").into_response()
        ),
    };
    // let current_time = Utc::now().date_naive();
    use base64::{engine::general_purpose::STANDARD, Engine as _};

    let t_64 = match STANDARD.decode(token) {
        Ok(claims) => claims,
        Err(_) => todo!(),
    };
    let k: Claims = bincode::deserialize(&t_64[..]).unwrap();

    let pr_list = slt(pool, k.start, k.end).await.unwrap();

    let mut context = Context::new();
    context.insert("k", &k);
    context.insert("pr_list", &pr_list);
    Ok(Html(templates.render("search_days", &context).unwrap()))
}
