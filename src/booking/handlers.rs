use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect},
    // http::{Request,Response,StatusCode},
    // body::Body,
    Extension,
};
// use chrono::Utc;

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use tera::Context;

use axum_extra::TypedHeader;
use headers::Cookie;

use crate::schema;
use crate::{
    common::{Pool, DBConnection, Templates},
    booking::models::{
        Claims,
        AllPrD,
        // LtBkg
    },
    booking::views::{all_bkg},
};

pub use axum_macros::debug_handler;

pub async fn get_all(
    DBConnection(conn): DBConnection,
    Extension(templates): Extension<Templates>
) -> impl IntoResponse {

    let bkg = all_bkg(DBConnection(conn)).await.unwrap();

    let mut context = Context::new();
    context.insert("bkg", &bkg);
    Html(templates.render("all_booking", &context).unwrap())
}
/*pub async fn get_all(
    DBConnection(mut conn): DBConnection,
    Extension(templates): Extension<Templates>
) -> impl IntoResponse {

    use schema::booking::dsl::*;
    let bkg = booking
        .select(LtBkg::as_select())
        .load(&mut conn)
        .await
        .unwrap();

    let mut context = Context::new();
    context.insert("bkg", &bkg);
    Html(templates.render("all_booking", &context).unwrap())
}*/


#[debug_handler]
pub async fn get_search_days(
    State(pool): State<Pool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    
    let token = match cookie.get("period") {
        Some(expr) => expr,
        None => return Err(Redirect::to("/booking/creat-period-days").into_response()),
    };

    use base64::{engine::general_purpose::STANDARD, Engine as _};

    let t_64 = match STANDARD.decode(token) {
        Ok(claims) => claims,
        Err(_) => todo!(),
    };
    let k: Claims = bincode::deserialize(&t_64[..]).unwrap();

    // let current_time = Utc::now().date_naive();

    let mut conn = pool.get().await.unwrap();
    use schema::provision_d::dsl::*;
    let pr_list = provision_d
        .filter(st_date.lt(k.start).and(en_date.gt(k.end)))
        .select(AllPrD::as_select())
        .load(&mut conn)
        .await
        .unwrap();

    let mut context = Context::new();
    context.insert("k", &k);
    context.insert("pr_list", &pr_list);
    Ok(Html(templates.render("all_days", &context).unwrap()))
}
