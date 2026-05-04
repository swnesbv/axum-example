use std::sync::Arc;
use axum::{
    extract::{Form, Path, State},
    response::{Html, IntoResponse, Redirect},
    http::{header::{HeaderMap}},
    Extension,
};
use chrono::{NaiveDate, NaiveDateTime, Utc};
use tera::Context;

use crate::{
    auth::models::{AuthRedis},
    common::{Templates, to_bool},
    provision::models::{FormStringPrd, FormPrH, UpPrD, UpPrH, ParsePointError},
    provision::dates::{one_update_prd, post_update_prd, del_prd},
    provision::hours::{one_update_prh, post_update_prh, del_prh}
};

pub async fn get_creat_days(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            context.insert("err", &err.to_string());
            return Ok(Html(templates.render("creat_days", &context).unwrap()));
        }
        Ok(None) | Err(None) => return Err(Redirect::to("/account/login").into_response()),
    };
    context.insert("t", &t);
    Ok(Html(templates.render("creat_days", &context).unwrap()))
}

pub async fn post_creat_days(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
    Form(f): Form<FormStringPrd>,
) -> impl IntoResponse {

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            let mut context = Context::new();
            context.insert("err", &err.to_string());
            return Err(Html(templates.render("creat_days", &context).unwrap()));
        }
        Ok(None) | Err(None) => return Ok(Redirect::to("/account/login").into_response()),
    };

    let start: Option<NaiveDate> = match f.st_date {
        Some(ref expr) => match Some(NaiveDate::parse_from_str(
                    expr.as_ref(), "%Y-%m-%d"
                )
            )
        {
            Some(Ok(res)) => Some(res),
            Some(Err(_)) => None,
            None => None
        }
        None => None
    };
    let end: Option<NaiveDate> = match f.en_date {
        Some(ref expr) => match Some(NaiveDate::parse_from_str(
                    expr.as_ref(), "%Y-%m-%d"
                )
            )
        {
            Some(Ok(res)) => Some(res),
            Some(Err(_)) => None,
            None => None
        }
        None => None
    };

    let utc = Some(Utc::now().date_naive());
    if start < utc || end < utc || start > end {
        let mut context = Context::new();
        context.insert(
            "err", &"Please enter proper dates..!".to_string()
        );
        return Err(Html(templates.render("creat_days", &context).unwrap()));
    }

    let pg = match i.pool.get().await{
        Ok(expr) => expr,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err", &err.to_string());
            return Err(Html(templates.render("creat_days", &context).unwrap()));
        }
    };
    let result = pg.execute(
        "INSERT INTO provision_d (user_id, title, description, st_date, en_date, created_at) VALUES ($1,$2,$3,$4,$5,now())",
        &[&t.id, &f.title, &f.description, &start, &end]
    ).await;
    match result {
        Ok(result) => result,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err", &err.to_string());
            return Err(Html(templates.render("creat_days", &context).unwrap()));
        }
    };
    Ok(Redirect::to("/provision/all-days").into_response())
}

pub async fn get_update_days(
    headers: HeaderMap,
    Path(prv_id): Path<String>,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            context.insert("err", &err.to_string());
            return Ok(Html(templates.render("update_dates", &context).unwrap()));
        }
        Ok(None) | Err(None) => return Err(Redirect::to("/account/login").into_response()),
    };
    let number = match prv_id.parse::<i32>().map_err(|_| ParsePointError) {
        Ok(expr) => expr,
        Err(err) => {
            context.insert("err", &err);
            return Ok(Html(templates.render("update_dates", &context).unwrap()))
        }
    };
    let one = match one_update_prd(i.pool.clone(), number).await {
        Ok(res) => res,
        Err(err) => {
            context.insert("err", &err);
            return Ok(Html(templates.render("update_dates", &context).unwrap()))
        }
    };

    context.insert("t", &t);
    context.insert("i", &one);
    Ok(Html(templates.render("update_dates", &context).unwrap()))
}

pub async fn post_update_days(
    headers: HeaderMap,
    Path(prv_id): Path<String>,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
    Form(f): Form<FormStringPrd>,
) -> impl IntoResponse {

    let _ = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            let mut context = Context::new();
            context.insert("err", &err.to_string());
            return Err(Html(templates.render("creat_days", &context).unwrap()));
        }
        Ok(None) | Err(None) => return Ok(Redirect::to("/account/login").into_response()),
    };
    let number = match prv_id.parse::<i32>().map_err(|_| ParsePointError) {
        Ok(expr) => expr,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err", &err);
            return Err(Html(templates.render("update_dates", &context).unwrap()))
        }
    };

    let one = one_update_prd(i.pool.clone(), number).await.unwrap();

    let st_val = f.st_date.as_deref().unwrap_or("");
    let en_val = f.en_date.as_deref().unwrap_or("");

    let start: Option<NaiveDate> = if !st_val.is_empty() {
        Some(NaiveDate::parse_from_str(st_val, "%Y-%m-%d").expect("REASON"))
    } else {
        one.st_date
    };
    let end: Option<NaiveDate> = if !en_val.is_empty() {
        Some(NaiveDate::parse_from_str(en_val, "%Y-%m-%d").expect("REASON"))
    } else {
        one.en_date
    };

    let c = match f.completed {
        Some(expr) => expr,
        None => false.to_string()
    };
    let cpt = to_bool(&c);
    let p = UpPrD {
        title:       f.title.to_owned(),
        description: f.description.to_owned(),
        st_date:     start,
        en_date:     end,
        completed:   cpt,
    };
    match post_update_prd(i.pool.clone(), p, number).await {
        Ok(res) => res,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err", &err);
            return Err(Html(templates.render("update_dates", &context).unwrap()))
        }
    };
    Ok(Redirect::to(&("/provision/detail-days/".to_owned() + &prv_id.to_string())).into_response())
}

pub async fn get_del_days(
    headers: HeaderMap,
    Path(prv_id): Path<String>,
    State(i): State<Arc<AuthRedis>>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Ok(None) | Err(Some(_)) => return Err(Redirect::to("/account/login").into_response()),
        Err(None) => return Err(Redirect::to("/account/login").into_response()),
    };

    let number = match prv_id.parse::<i32>().map_err(|_| ParsePointError) {
        Ok(expr) => expr,
        Err(_) => return Err(Redirect::to("/account/login").into_response())
    };

    match del_prd(i.pool.clone(), number, t.id).await {
        Ok(res) => res,
        Err(_) => return Err(Redirect::to("/account/login").into_response())
    };

    Ok(Redirect::to("/provision/all-days").into_response())
}


// Hours..
pub async fn get_creat_hours(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            context.insert("err", &err.to_string());
            return Ok(Html(templates.render("creat_hours", &context).unwrap()));
        }
        Ok(None) | Err(None) => return Err(Redirect::to("/account/login").into_response()),
    };

    context.insert("t", &t);
    Ok(Html(templates.render("creat_hours", &context).unwrap()))
}

pub async fn post_creat_hours(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
    Form(f): Form<FormPrH>,
) -> impl IntoResponse {

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            let mut context = Context::new();
            context.insert("err", &err);
            return Err(Html(templates.render("creat_hours", &context).unwrap()))
        }
        Ok(None) | Err(None) => return Ok(Redirect::to("/account/login").into_response())
    };
    let pg = match i.pool.get().await{
        Ok(expr) => expr,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err", &err.to_string());
            return Err(Html(templates.render("creat_hours", &context).unwrap()))
        }
    };

    let s_val = f.st_hour.as_deref().unwrap_or("default string");
    let e_val = f.en_hour.as_deref().unwrap_or("default string");

    let start: Option<NaiveDateTime> = if !s_val.is_empty() {
        Some(NaiveDateTime::parse_from_str(s_val, "%Y-%m-%dT%H:%M").unwrap())
    } else {
        None
    };
    let end: Option<NaiveDateTime> = if !e_val.is_empty() {
        Some(NaiveDateTime::parse_from_str(e_val, "%Y-%m-%dT%H:%M").unwrap())
    } else {
        None
    };

    let utc = Some(Utc::now().naive_utc());
    if start < utc || end < utc || start > end {
        let mut context = Context::new();
        context.insert(
            "err", &"Please enter proper dates..!".to_string()
        );
        return Err(Html(templates.render("creat_days", &context).unwrap()));
    }

    let result = pg.execute(
        "INSERT INTO provision_h (user_id, title, description, st_hour, en_hour, created_at) VALUES ($1,$2,$3,$4,$5,now())",
        &[&t.id, &f.title, &f.description, &start, &end]
    ).await;
    match result {
        Ok(result) => result,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err", &err.to_string());
            return Err(Html(templates.render("creat_hours", &context).unwrap()));
        }
    };
    Ok(Redirect::to("/provision/all-hours").into_response())
}

pub async fn get_update_hours(
    headers: HeaderMap,
    Path(prv_id): Path<String>,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            context.insert("err", &err.to_string());
            return Ok(Html(templates.render("update_hours", &context).unwrap()));
        }
        Ok(None) | Err(None) => return Err(Redirect::to("/account/login").into_response()),
    };
    let number = match prv_id.parse::<i32>().map_err(|_| ParsePointError) {
        Ok(expr) => expr,
        Err(err) => {
            context.insert("err", &err);
            return Ok(Html(templates.render("update_hours", &context).unwrap()))
        }
    };
    let one = match one_update_prh(i.pool.clone(), number).await {
        Ok(res) => res,
        Err(err) => {
            context.insert("err", &err);
            return Ok(Html(templates.render("update_hours", &context).unwrap()))
        }
    };

    context.insert("t", &t);
    context.insert("i", &one);
    Ok(Html(templates.render("update_hours", &context).unwrap()))
}

pub async fn post_update_hours(
    headers: HeaderMap,
    Path(prv_id): Path<String>,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
    Form(f): Form<FormPrH>,
) -> impl IntoResponse {

    let _ = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            let mut context = Context::new();
            context.insert("err", &err.to_string());
            return Err(Html(templates.render("update_hours", &context).unwrap()));
        }
        Ok(None) | Err(None) => return Ok(Redirect::to("/account/login").into_response()),
    };
    let number = match prv_id.parse::<i32>().map_err(|_| ParsePointError) {
        Ok(expr) => expr,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err", &err);
            return Err(Html(templates.render("update_hours", &context).unwrap()))
        }
    };

    let one = one_update_prh(i.pool.clone(), number).await.unwrap();

    let s_val = f.st_hour.as_deref().unwrap_or("default string");
    let e_val = f.en_hour.as_deref().unwrap_or("default string");

    let start: Option<NaiveDateTime> = if !s_val.is_empty() {
        Some(NaiveDateTime::parse_from_str(s_val, "%Y-%m-%dT%H:%M").unwrap())
    } else {
        one.st_hour
    };
    let end: Option<NaiveDateTime> = if !e_val.is_empty() {
        Some(NaiveDateTime::parse_from_str(e_val, "%Y-%m-%dT%H:%M").unwrap())
    } else {
        one.en_hour
    };

    let c = match f.completed {
        Some(expr) => expr,
        None => false.to_string()
    };
    let cpt = to_bool(&c);
    let p = UpPrH {
        title:       f.title.to_owned(),
        description: f.description.to_owned(),
        st_hour:     start,
        en_hour:     end,
        completed:   cpt,
    };
    match post_update_prh(i.pool.clone(), p, number).await {
        Ok(res) => res,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err", &err);
            return Err(Html(templates.render("update_hours", &context).unwrap()))
        }
    };
    Ok(Redirect::to(&("/provision/detail-hours/".to_owned() + &prv_id.to_string())).into_response())
}

pub async fn get_del_hours(
    headers: HeaderMap,
    Path(prv_id): Path<String>,
    State(i): State<Arc<AuthRedis>>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Ok(None) | Err(Some(_)) => return Err(Redirect::to("/account/login").into_response()),
        Err(None) => return Err(Redirect::to("/account/login").into_response()),
    };

    let number = match prv_id.parse::<i32>().map_err(|_| ParsePointError) {
        Ok(expr) => expr,
        Err(_) => return Err(Redirect::to("/account/login").into_response())
    };

    match del_prh(i.pool.clone(), number, t.id).await {
        Ok(res) => res,
        Err(_) => return Err(Redirect::to("/account/login").into_response())
    };

    Ok(Redirect::to("/provision/all-hours").into_response())
}


    // let st_val = f.st_date.as_deref().unwrap_or("default string");
    // let en_val = f.en_date.as_deref().unwrap_or("default string");
    // let start: Option<NaiveDate> = if !st_val.is_empty() {
    //     Some(NaiveDate::parse_from_str(st_val, "%Y-%m-%d").expect("REASON"))
    // } else {
    //     None
    // };
    // let end: Option<NaiveDate> = if !en_val.is_empty() {
    //     Some(NaiveDate::parse_from_str(en_val, "%Y-%m-%d").expect("REASON"))
    // } else {
    //     None
    // };