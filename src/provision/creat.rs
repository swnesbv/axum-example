use sqlx::postgres::PgPool;

use axum::{
    extract::{Form, Path, State},
    response::{Html, IntoResponse, Redirect},
    Extension,
};

use chrono::{NaiveDate, NaiveDateTime, Utc};

use tera::Context;

use axum_extra::TypedHeader;
use headers::Cookie;

use crate::auth;
use crate::{
    common::{DatabaseConn, Templates},
    provision::models::{
        FormPrD,
        FormPrH,
        UpPrD,
    },
    provision::views::{list_update_prd, post_update_prd},
};

pub async fn get_creat_days(
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let token = auth::views::request_user(cookie).await;
    let _ = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(_) => return Err(Redirect::to("/account/login").into_response()),
    };
    Ok(Html(
        templates.render("creat_days", &Context::new()).unwrap(),
    ))
}

pub async fn post_creat_days(
    State(pool): State<PgPool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
    Form(form): Form<FormPrD>,
) -> impl IntoResponse {

    let token = auth::views::request_token(cookie).await.unwrap();

    let s_value = form.st_date.as_deref().unwrap_or("default string");
    let e_value = form.en_date.as_deref().unwrap_or("default string");

    let start: Option<NaiveDate> = if !s_value.is_empty() {
        Some(NaiveDate::parse_from_str(s_value, "%Y-%m-%d").expect("REASON"))
    } else {
        None
    };

    let end: Option<NaiveDate> = if !e_value.is_empty() {
        Some(NaiveDate::parse_from_str(e_value, "%Y-%m-%d").expect("REASON"))
    } else {
        None
    };

    let result = sqlx::query(
        "INSERT INTO provision_d (user_id, title, description, st_date, en_date, created_at) VALUES ($1,$2,$3,$4,$5,$6)"
        )
        .bind(token.claims.id)
        .bind(&form.title)
        .bind(&form.description)
        .bind(start)
        .bind(end)
        .bind(Utc::now())
        .execute(&pool)
        .await;
    match result {
        Ok(result) => result,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err_token", &err.to_string());
            return Err(Html(templates.render("creat_days", &context).unwrap()));
        }
    };
    Ok(Redirect::to("/provision/all-days").into_response())
}

pub async fn get_update_days(
    Path(prv_id): Path<String>,
    DatabaseConn(mut conn): DatabaseConn,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let token = auth::views::request_user(cookie).await;
    let _ = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(_) => return Err(Redirect::to("/account/login").into_response()),
    };

    let number: i32 = prv_id.parse().expect("Not a valid number");

    let i = list_update_prd(&mut conn, number).await.unwrap();

    let mut context = Context::new();
    context.insert("i", &i);
    Ok(Html(templates.render("update_dates", &context).unwrap()))
}

pub async fn post_update_days(
    Path(prv_id): Path<String>,
    DatabaseConn(mut conn): DatabaseConn,
    Form(form): Form<FormPrD>,
) -> impl IntoResponse {

    let number: i32 = prv_id.parse().expect("Not a valid number");

    let i = list_update_prd(&mut conn, number).await.unwrap();

    let s_value = form.st_date.as_deref().unwrap_or("default string");
    let e_value = form.en_date.as_deref().unwrap_or("default string");

    let start: Option<NaiveDate> = if !s_value.is_empty() {
        Some(NaiveDate::parse_from_str(s_value, "%Y-%m-%d").expect("REASON"))
    } else {
        i.st_date
    };

    let end: Option<NaiveDate> = if !e_value.is_empty() {
        Some(NaiveDate::parse_from_str(e_value, "%Y-%m-%d").expect("REASON"))
    } else {
        i.en_date
    };

    let p = UpPrD {
        title: form.title.to_owned(),
        description: form.description.to_owned(),
        st_date: start,
        en_date: end,
        updated_at: Some(Utc::now()),
    };
    let result = post_update_prd(&mut conn, number, p).await;
    let _ = match result {
        Ok(result) => Ok(result),
        Err(err) => Err(err.to_string()),
    };

    Redirect::to(&("/provision/detail-days/".to_owned() + &prv_id.to_string())).into_response()
}

// Hours..
pub async fn get_creat_hours(
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let token = auth::views::request_user(cookie).await;
    let _ = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(_) => return Err(Redirect::to("/account/login").into_response()),
    };
    Ok(Html(
        templates.render("creat_hours", &Context::new()).unwrap(),
    ))
}

pub async fn post_creat_hours(
    State(pool): State<PgPool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
    Form(form): Form<FormPrH>,
) -> impl IntoResponse {
    let token = auth::views::request_token(cookie).await.unwrap();

    let s_value = form.st_hour.as_deref().unwrap_or("default string");
    let e_value = form.en_hour.as_deref().unwrap_or("default string");

    let start: Option<NaiveDateTime> = if !s_value.is_empty() {
        Some(NaiveDateTime::parse_from_str(s_value, "%Y-%m-%dT%H:%M").unwrap())
    } else {
        None
    };
    let end: Option<NaiveDateTime> = if !e_value.is_empty() {
        Some(NaiveDateTime::parse_from_str(e_value, "%Y-%m-%dT%H:%M").unwrap())
    } else {
        None
    };

    let result = sqlx::query(
        "INSERT INTO provision_h (user_id, title, description, st_hour, en_hour, created_at) VALUES ($1,$2,$3,$4,$5,$6)"
        )
        .bind(token.claims.id)
        .bind(&form.title)
        .bind(&form.description)
        .bind(start)
        .bind(end)
        .bind(Utc::now())
        .execute(&pool)
        .await;
    match result {
        Ok(result) => result,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err_token", &err.to_string());
            return Err(Html(templates.render("creat_hours", &context).unwrap()));
        }
    };
    Ok(Redirect::to("/provision/all-hours").into_response())
}
