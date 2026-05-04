use std::sync::Arc;
use axum::{
    extract::{Form, Path, State, OriginalUri},
    response::{Html, IntoResponse, Redirect},
    http::{header::{HeaderMap}},
    Extension,
};
use chrono::{NaiveDate, NaiveDateTime};
use tera::Context;

use crate::{
    common::{Templates},
    auth::models::{AuthRedis},
    comments::models::{FormComment},
    comments::views::{list_cmt},
    booking::dates::{check_period_days},
    booking::hours::{check_period_hours},
    provision::dates::{creat_bkg_days},
    provision::hours::{creat_bkg_hours},
    provision::repository::{all_days, all_hours, details_prd, details_prh},
    provision::models::{BkgPrD, BkgPrH, Prd, Prh, ParsePointError}
};

pub async fn get_detail_days(
    headers: HeaderMap,
    Path(prv_id): Path<String>,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            context.insert("err", &err);
            return Err(Html(templates.render("detail_days", &context).unwrap()))
        }
        Ok(None) | Err(None) => {
            context.insert("err", "Caramba bullfighting and damn it");
            return Err(Html(templates.render("detail_days", &context).unwrap()))
        }
    };

    let number = match prv_id.parse::<i32>().map_err(|_| ParsePointError) {
        Ok(expr) => expr,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err", &err);
            return Ok(Html(templates.render("detail_days", &context).unwrap()))
        }
    };

    let d = match details_prd(i.pool.clone(), number).await {
        Ok(expr) => expr,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err", &err);
            return Ok(Html(templates.render("detail_days", &context).unwrap()))
        }
    };

    let cmt = list_cmt(i.pool.clone(), number, "provision_d").await;
    let _ = match cmt {
        Ok(expr) => {
            context.insert("cmt", &expr);
            Ok(Html(templates.render("detail_days", &context).unwrap()))
        }
        Err(Some(err)) => {
            context.insert("err", &err.to_string());
            Err(Html(templates.render("detail_days", &context).unwrap()))
        }
        Err(None) => {
            context.insert("is_no", "Caramba bullfighting and damn it");
            Err(Html(templates.render("detail_days", &context).unwrap()))
        }
    };

    context.insert("t", &t);
    context.insert("i", &d);
    Ok(Html(templates.render("detail_days", &context).unwrap()))
}

pub async fn post_detail_days(
    headers: HeaderMap,
    Path(prv_id): Path<String>,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
    OriginalUri(original_uri): OriginalUri,
    Form(f): Form<Prd>,
) -> impl IntoResponse {

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            let mut context = Context::new();
            context.insert("err", &err);
            return Err(Html(templates.render("detail_days", &context).unwrap()))
        }
        Ok(None) | Err(None) => {
            let mut context = Context::new();
            context.insert("is_no", "Caramba bullfighting and damn it");
            return Err(Html(templates.render("detail_days", &context).unwrap()))
        }
    };
    let number = match prv_id.parse::<i32>().map_err(|_| ParsePointError) {
        Ok(expr) => expr,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err", &err);
            return Err(Html(templates.render("detail_days", &context).unwrap()))
        }
    };
    let pg = match i.pool.get().await{
        Ok(expr) => expr,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err", &err.to_string());
            return Err(Html(templates.render("detail_days", &context).unwrap()))
        }
    };

    if f.title.is_some() {
        let title       = f.title;
        let description = f.description;
        let start       = f.s_dates;
        let end         = f.e_dates;

        if check_period_days(i.pool.clone(), start, end, number).await.unwrap() {
            let b = BkgPrD {
                user_id:        t.id,
                provision_d_id: Some(number),
                title:          title.to_owned().unwrap_or_default(),
                description:    description.to_owned(),
                st_date:        start,
                en_date:        end,
            };
            let _ = creat_bkg_days(i.pool.clone(), b).await.unwrap();
        } else {
            let mut context = Context::new();
            context.insert(
                "err", &"the correspondence of the entered period does not match the date attachments"
            );
            return Err(Html(templates.render("search_days", &context).unwrap()))
        }

        let zero_date = NaiveDate::parse_from_str(
            "0001-01-01", "%Y-%m-%d"
        ).unwrap_or_default();
        if start.is_none() & end.is_none() {
            let _ = Redirect::to("/booking/all-booking").into_response();
        }
        if start.is_some() & end.is_some() {
            let s_vec = vec![start];
            let e_vec = vec![end];
            let d_vec = vec![start, end];
            let result = pg.execute(
                "UPDATE provision_d SET s_dates=ARRAY_CAT(s_dates, $2), e_dates=ARRAY_CAT(e_dates, $3), dates=ARRAY_CAT(dates, $4), updated_at=now() WHERE id=$1",
                &[&number, &s_vec, &e_vec, &d_vec]
            ).await;
            let _ = match result {
                Ok(expr) => Ok(expr),
                Err(err) => Err(err.to_string()),
            };
        }
        if start.is_some() & end.is_none() {
            let s_vec = vec![start];
            let e_vec = vec![zero_date];
            let d_vec = vec![start, Some(zero_date)];
            let result = pg.execute(
                "UPDATE provision_d SET s_dates=ARRAY_CAT(s_dates, $2), e_dates=ARRAY_CAT(e_dates, $3), dates=ARRAY_CAT(dates, $4), updated_at=now() WHERE id=$1",
                &[&number, &s_vec, &e_vec, &d_vec]
            ).await;
            let _ = match result {
                Ok(expr) => Ok(expr),
                Err(err) => Err(err.to_string()),
            };
        }
        if start.is_none() & end.is_some() {
            let s_vec = vec![zero_date];
            let e_vec = vec![end];
            let d_vec = vec![zero_date, end.unwrap_or_default()];
            let result = pg.execute(
                "UPDATE provision_d SET s_dates=ARRAY_CAT(s_dates, $2), e_dates=ARRAY_CAT(e_dates, $3), dates=ARRAY_CAT(dates, $4), updated_at=now() WHERE id=$1",
                &[&number, &s_vec, &e_vec, &d_vec]
            ).await;
            let _ = match result {
                Ok(expr) => Ok(expr),
                Err(err) => Err(err.to_string()),
            };
        }
        return Ok(Redirect::to("/booking/all-booking").into_response());
    };

    if f.comment.is_some() {
        let pat = FormComment {
            to_id: f.to_id, comment: f.comment
        };
        let result = pat.insert_cmt(
            i.pool.clone(), t.id, t.email, t.username, "provision_d"
        ).await;
        let _ = match result {
            Ok(expr) => Ok(expr),
            Err(Some(err)) => Err(err.to_string()),
            Err(None) => {
                let mut context = Context::new();
                context.insert("is_no", "Caramba bullfighting and damn it");
                return Err(Html(templates.render("detail_days", &context).unwrap()))
            }
        };
    }
    Ok(Redirect::to(original_uri.path()).into_response())
}


pub async fn get_detail_hours(
    headers: HeaderMap,
    Path(prv_id): Path<String>,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            context.insert("err", &err);
            return Err(Html(templates.render("detail_hours", &context).unwrap()))
        }
        Ok(None) | Err(None) => {
            context.insert("err", "Caramba bullfighting and damn it");
            return Err(Html(templates.render("detail_hours", &context).unwrap()))
        }
    };
    let number = match prv_id.parse::<i32>().map_err(|_| ParsePointError) {
        Ok(expr) => expr,
        Err(err) => {
            context.insert("err", &err);
            return Ok(Html(templates.render("detail_hours", &context).unwrap()))
        }
    };

    let d = match details_prh(i.pool.clone(), number).await {
        Ok(expr) => expr,
        Err(err) => {
            context.insert("err", &err);
            return Ok(Html(templates.render("detail_hours", &context).unwrap()))
        }
    };

    let cmt = list_cmt(i.pool.clone(), number, "provision_h").await;
    let _ = match cmt {
        Ok(expr) => {
            context.insert("cmt", &expr);
            Ok(Html(templates.render("detail_hours", &context).unwrap()))
        }
        Err(Some(err)) => {
            context.insert("err", &err.to_string());
            Err(Html(templates.render("detail_hours", &context).unwrap()))
        }
        Err(None) => {
            context.insert("is_no", "Caramba bullfighting and damn it");
            Err(Html(templates.render("detail_hours", &context).unwrap()))
        }
    };
    context.insert("t", &t);
    context.insert("i", &d);
    Ok(Html(templates.render("detail_hours", &context).unwrap()))
}

pub async fn post_detail_hours(
    headers: HeaderMap,
    Path(prv_id): Path<String>,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
    OriginalUri(original_uri): OriginalUri,
    Form(f): Form<Prh>
) -> impl IntoResponse {

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            let mut context = Context::new();
            context.insert("err", &err);
            return Err(Html(templates.render("detail_hours", &context).unwrap()))
        }
        Ok(None) | Err(None) => {
            let mut context = Context::new();
            context.insert("is_no", "Caramba bullfighting and damn it");
            return Err(Html(templates.render("detail_days", &context).unwrap()))
        }
    };
    let number = match prv_id.parse::<i32>().map_err(|_| ParsePointError) {
        Ok(expr) => expr,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err", &err);
            return Err(Html(templates.render("detail_hours", &context).unwrap()))
        }
    };
    let pg = match i.pool.get().await{
        Ok(expr) => expr,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err", &err.to_string());
            return Err(Html(templates.render("detail_hours", &context).unwrap()))
        }
    };

    if f.title.is_some() {
        let title       = f.title;
        let description = f.description;
        let start       = f.s_hours;
        let end         = f.e_hours;

        if check_period_hours(i.pool.clone(), start, end, number).await.unwrap() {
            let b = BkgPrH {
                user_id:        t.id,
                provision_h_id: Some(number),
                title:          title.to_owned().unwrap_or_default(),
                description:    description.to_owned(),
                st_hour:        start,
                en_hour:        end,
            };
            let _ = creat_bkg_hours(i.pool.clone(), b).await.unwrap();
        } else {
            let mut context = Context::new();
            context.insert(
                "err", &"the correspondence of the entered period does not match the date attachments"
            );
            return Err(Html(templates.render("detail_hours", &context).unwrap()))
        }
        let zero_date = NaiveDateTime::parse_from_str(
            "0001-01-01", "%Y-%m-%d"
        ).unwrap_or_default();

        if start.is_none() & end.is_none() {
            let _ = Redirect::to("/booking/all-booking").into_response();
        }
        if start.is_some() & end.is_some() {
            let s_vec = vec![start];
            let e_vec = vec![end];
            let d_vec = vec![start, end];
            let result = pg.execute(
                "UPDATE provision_h SET s_dates=ARRAY_CAT(s_hours, $2), e_hours=ARRAY_CAT(e_hours, $3), hours=ARRAY_CAT(hours, $4), updated_at=now() WHERE id=$1",
                &[&number, &s_vec, &e_vec, &d_vec]
            ).await;
            let _ = match result {
                Ok(result) => Ok(result),
                Err(err) => Err(err.to_string()),
            };
        }
        if start.is_some() & end.is_none() {
            let s_vec = vec![start];
            let e_vec = vec![zero_date];
            let d_vec = vec![start, Some(zero_date)];
            let result = pg.execute(
                "UPDATE provision_h SET s_dates=ARRAY_CAT(s_hours, $2), e_hours=ARRAY_CAT(e_dates, $3), hours=ARRAY_CAT(hours, $4), updated_at=now() WHERE id=$1",
                &[&number, &s_vec, &e_vec, &d_vec]
            ).await;
            let _ = match result {
                Ok(result) => Ok(result),
                Err(err) => Err(err.to_string()),
            };
        }
        if start.is_none() & end.is_some() {
            let s_vec = vec![zero_date];
            let e_vec = vec![end];
            let d_vec = vec![zero_date, end.unwrap_or_default()];
            let result = pg.execute(
                "UPDATE provision_h SET s_dates=ARRAY_CAT(s_hours, $2), e_hours=ARRAY_CAT(e_hours, $3), hours=ARRAY_CAT(hours, $4), updated_at=now() WHERE id=$1",
                &[&number, &s_vec, &e_vec, &d_vec]
            ).await;
            let _ = match result {
                Ok(result) => Ok(result),
                Err(err) => Err(err.to_string()),
            };
        }
        return Ok(Redirect::to("/booking/all-booking").into_response());
    };

    if f.comment.is_some() {
        let pat = FormComment {
            to_id: f.to_id, comment: f.comment
        };
        let result = pat.insert_cmt(
            i.pool.clone(), t.id, t.email, t.username, "provision_h"
        ).await;
        let _ = match result {
            Ok(expr) => Ok(expr),
            Err(Some(err)) => Err(err.to_string()),
            Err(None) => {
                let mut context = Context::new();
                context.insert("is_no", "Caramba bullfighting and damn it");
                return Err(Html(templates.render("detail_hours", &context).unwrap()))
            }
        };
    }
    Ok(Redirect::to(original_uri.path()).into_response())
}


pub async fn get_all_days(
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> impl IntoResponse {

    let all = all_days(i.pool.clone()).await.unwrap();

    let mut context = Context::new();
    context.insert("all", &all);
    Html(templates.render("all_days", &context).unwrap())
}

pub async fn get_all_hours(
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> impl IntoResponse {

    let all = all_hours(i.pool.clone()).await.unwrap();

    let mut context = Context::new();
    context.insert("all", &all);
    Html(templates.render("all_hours", &context).unwrap())
}