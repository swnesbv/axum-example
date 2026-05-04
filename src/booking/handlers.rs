use std::sync::Arc;
use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect},
    http::{header::{HeaderMap}},
    Extension,
};
use tera::Context;
use chrono::{NaiveDate, NaiveDateTime};

use crate::{
    common::{Templates, to_token},
    auth::models::{AuthRedis},
    booking::dates::{check_prd_list, check_prd_start, check_prd_end},
    booking::hours::{check_prh_list, check_prh_start, check_prh_end},
    booking::models::{CheckListPrD, CheckListPrH},
    booking::repository::{all}
};


pub async fn get_matching_days(
    Extension(templates): Extension<Templates>
) -> impl IntoResponse {
    Html(templates.render("matching_days", &Context::new()).unwrap())
}

pub async fn get_matching_hours(
    Extension(templates): Extension<Templates>
) -> impl IntoResponse {
    Html(templates.render("matching_hours", &Context::new()).unwrap())
}


pub async fn get_search_days(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let token = match to_token(headers, "period".to_string()).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            context.insert("err", &err);
            return Ok(Html(templates.render("search_days", &context).unwrap()))
        }
        Ok(None) | Err(None) => return Err(Redirect::to("/booking/matching-days").into_response())
    };

    let obj: Vec<&str> = token.split(",").collect();
    let mut v: Vec<NaiveDate> = Vec::new();
    for i in obj {
        if !i.is_empty() {
            v.push(
                NaiveDate::parse_from_str(i, "%Y-%m-%d").unwrap()
            )
        }
    }
    let a: Option<NaiveDate> = v.first().copied();
    let b: Option<NaiveDate> = v.get(1).copied();
    let mut pr_list = vec!(CheckListPrD::default());

    if a.is_some() & b.is_some() {
        pr_list = match check_prd_list(i.pool.clone(), a, b).await {
            Ok(Some(ref expr)) => expr.to_vec(),
            Err(Some(err)) => {
                context.insert("err", &err);
                return Ok(Html(templates.render("search_days", &context).unwrap()))
            }
            Ok(None) | Err(None) => {
                context.insert("err", &"Caramba bullfighting and damn it..!");
                return Ok(Html(templates.render("search_days", &context).unwrap()))
            }
        }
    };
    if a.is_some() & b.is_none() {
        pr_list = match check_prd_start(i.pool.clone(), a).await {
            Ok(Some(ref expr)) => expr.to_vec(),
            Err(Some(err)) => {
                context.insert("err", &err);
                return Ok(Html(templates.render("search_days", &context).unwrap()))
            }
            Ok(None) | Err(None) => {
                context.insert("err", &"Caramba bullfighting and damn it..!");
                return Ok(Html(templates.render("search_days", &context).unwrap()))
            }
        }
    };
    if a.is_none() & b.is_some() {
        pr_list = match check_prd_end(i.pool.clone(), b).await {
            Ok(Some(ref expr)) => expr.to_vec(),
            Err(Some(err)) => {
                context.insert("err", &err);
                return Ok(Html(templates.render("search_days", &context).unwrap()))
            }
            Ok(None) | Err(None) => {
                context.insert("err", &"Caramba bullfighting and damn it..!");
                return Ok(Html(templates.render("search_days", &context).unwrap()))
            }
        }
    };

    context.insert("v", &v);
    context.insert("pr_list", &pr_list);
    Ok(Html(templates.render("search_days", &context).unwrap()))
}

pub async fn get_search_hours(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let token = match to_token(headers, "period".to_string()).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            context.insert("err", &err);
            return Ok(Html(templates.render("search_hours", &context).unwrap()))
        }
        Ok(None) | Err(None) => return Err(Redirect::to("/booking/matching-days").into_response())
    };

    let obj: Vec<&str> = token.split(",").collect();
    let mut v: Vec<NaiveDateTime> = Vec::new();
    for i in obj {
        if !i.is_empty() {
            v.push(
                NaiveDateTime::parse_from_str(i, "%Y-%m-%dT%H:%M").unwrap()
            )
        }
    }
    let a: Option<NaiveDateTime> = v.first().copied();
    let b: Option<NaiveDateTime> = v.get(1).copied();
    let mut pr_list = vec!(CheckListPrH::default());

    if a.is_some() & b.is_some() {
        pr_list = match check_prh_list(i.pool.clone(), a, b).await {
            Ok(Some(ref expr)) => expr.to_vec(),
            Err(Some(err)) => {
                context.insert("err", &err);
                return Ok(Html(templates.render("search_hours", &context).unwrap()))
            }
            Ok(None) | Err(None) => {
                context.insert("err", &"Caramba bullfighting and damn it..!");
                return Ok(Html(templates.render("search_hours", &context).unwrap()))
            }
        }
    };
    if a.is_some() & b.is_none() {
        pr_list = match check_prh_start(i.pool.clone(), a).await {
            Ok(Some(ref expr)) => expr.to_vec(),
            Err(Some(err)) => {
                context.insert("err", &err);
                return Ok(Html(templates.render("search_hours", &context).unwrap()))
            }
            Ok(None) | Err(None) => {
                context.insert("err", &"Caramba bullfighting and damn it..!");
                return Ok(Html(templates.render("search_hours", &context).unwrap()))
            }
        }
    };
    if a.is_none() & b.is_some() {
        pr_list = match check_prh_end(i.pool.clone(), b).await {
            Ok(Some(ref expr)) => expr.to_vec(),
            Err(Some(err)) => {
                context.insert("err", &err);
                return Ok(Html(templates.render("search_hours", &context).unwrap()))
            }
            Ok(None) | Err(None) => {
                context.insert("err", &"Caramba bullfighting and damn it..!");
                return Ok(Html(templates.render("search_hours", &context).unwrap()))
            }
        }
    };

    context.insert("v", &v);
    context.insert("pr_list", &pr_list);
    Ok(Html(templates.render("search_hours", &context).unwrap()))
}


pub async fn bkg_all(
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> impl IntoResponse {

    let bkg = all(i.pool.clone()).await.unwrap();

    let mut context = Context::new();
    context.insert("bkg", &bkg);
    Html(templates.render("all_booking", &context).unwrap())
}