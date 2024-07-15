use sqlx::postgres::PgPool;

use axum::{
    extract::{Form, Path, State},
    response::{Html, IntoResponse, Redirect},
    Extension,
};
use chrono::{Utc, NaiveDate};

use tera::Context;

use axum_extra::TypedHeader;
use headers::Cookie;

use crate::{auth};
use crate::{
    common::{Templates},
    provision::models::{
        FormPrdBkg, BkgPrD,
    },
    provision::views::{creat_bkg, all_days,
        details,
        // update_prv
    },
};


pub async fn get_all_days(
    State(pool): State<PgPool>,
    Extension(templates): Extension<Templates>
) -> impl IntoResponse {

    let all = all_days(pool).await.unwrap();

    let mut context = Context::new();
    context.insert("all", &all);
    Html(templates.render("all_days", &context).unwrap())
}


pub async fn get_detail_days(
    Path(prv_id): Path<String>,
    State(pool): State<PgPool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    
    let token = auth::views::request_user(TypedHeader(cookie)).await;
    let _ = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(_) => return Err(Redirect::to("/account/login").into_response()),
    };
    let number: i32 = prv_id.parse().expect("Not a valid number");

    let i = details(pool, number).await.unwrap();

    let mut context = Context::new();
    context.insert("i", &i);
    Ok(Html(templates.render("detail_days", &context).unwrap()))
}


pub async fn post_detail_days(
    Path(prv_id): Path<String>,
    State(pool): State<PgPool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Form(form): Form<FormPrdBkg>,
) -> impl IntoResponse {

    let number: i32 = prv_id.parse().expect("Not a valid number");

    let token = auth::views::request_token(TypedHeader(cookie)).await;
    let owner = token.clone().unwrap().claims.id;

    let start = Some(form.s_dates);
    let end   = Some(form.e_dates);
    println!("start.. {:?}", start);

    let b = BkgPrD {
        user_id: owner,
        provision_d_id: Some(number),
        title: form.title.clone(),
        description: form.description.clone(),
        st_date: start,
        en_date: end,
        created_at: Utc::now(),
    };
    let _ = creat_bkg(pool.clone(), b).await.unwrap();


    let s_val = form.s_dates;
    let e_val = form.e_dates;

    let zero_date = NaiveDate::parse_from_str(
        "0001-01-01", "%Y-%m-%d"
    ).expect("msg");

    if start.is_none() && end.is_none() {
        Redirect::to("/booking/all-booking").into_response();
    }
    if start.is_some() && end.is_some() {
        let s_vec = vec![s_val];
        let e_vec = vec![e_val];
        let d_vec = vec![s_val, e_val];
        let result = sqlx::query!(
            "UPDATE provision_d SET s_dates=ARRAY_CAT(s_dates, $2), e_dates=ARRAY_CAT(e_dates, $3), dates=ARRAY_CAT(dates, $4), updated_at=$5 WHERE id=$1", number, &s_vec, &e_vec, &d_vec, Some(Utc::now())
            )
            .fetch_one(&pool).await;
        let _ = match result {
            Ok(result) => Ok(result),
            Err(err) => Err(err.to_string()),
        };
    }

    if start.is_some() && end.is_none() {
        let s_vec = vec![s_val];
        let e_vec = vec![zero_date];
        let d_vec = vec![s_val, zero_date];
        let result = sqlx::query!(
            "UPDATE provision_d SET s_dates=ARRAY_CAT(s_dates, $2), e_dates=ARRAY_CAT(e_dates, $3), dates=ARRAY_CAT(dates, $4), updated_at=$5 WHERE id=$1", number, &s_vec, &e_vec, &d_vec, Some(Utc::now())
            )
            .fetch_one(&pool).await;
        let _ = match result {
            Ok(result) => Ok(result),
            Err(err) => Err(err.to_string()),
        };
    }
    if start.is_none() && end.is_some() {
        let s_vec = vec![zero_date];
        let e_vec = vec![e_val];
        let d_vec = vec![zero_date, e_val];
        let result = sqlx::query!(
            "UPDATE provision_d SET s_dates=ARRAY_CAT(s_dates, $2), e_dates=ARRAY_CAT(e_dates, $3), dates=ARRAY_CAT(dates, $4), updated_at=$5 WHERE id=$1", number, &s_vec, &e_vec, &d_vec, Some(Utc::now())
            )
            .fetch_one(&pool).await;
        let _ = match result {
            Ok(result) => Ok(result),
            Err(err) => Err(err.to_string()),
        };
    }

    Redirect::to("/booking/all-booking").into_response()
}
