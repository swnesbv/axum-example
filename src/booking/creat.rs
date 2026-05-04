use std::sync::Arc;
use axum::{
    body::Body,
    extract::{Path, Form, State},
    http::{Response, StatusCode},
    response::{Html, Redirect, IntoResponse},
    http::header::{HeaderMap},
    Extension,
};
use chrono::{NaiveDate, NaiveDateTime,Utc};
use tera::Context;

use crate::{
    common::{Templates},
    auth::models::AuthRedis,
    booking::models::{FormSearchPrd},
    booking::dates::{check_period_days},
    booking::hours::{check_period_hours},
    provision::models::{
        FormCreatBkgD, FormCreatBkgH, BkgPrD, BkgPrH, ParsePointError
    },
    provision::dates::{creat_bkg_days},
    provision::hours::{creat_bkg_hours}
};

pub async fn post_matching_days(
    Extension(templates): Extension<Templates>,
    Form(f): Form<FormSearchPrd>
) -> impl IntoResponse {

    let start: Option<NaiveDate> = match f.start {
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
    let end: Option<NaiveDate> = match f.end {
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

    if start.is_some() & end.is_none() && start < utc {
        let mut context = Context::new();
        context.insert(
            "err", &"Please enter proper dates..!".to_string()
        );
        return Err(Html(templates.render("matching_days", &context).unwrap()));
    }
    if start.is_none() & end.is_some() && end < utc {
        let mut context = Context::new();
        context.insert(
            "err", &"Please enter proper dates..!".to_string()
        );
        return Err(Html(templates.render("matching_days", &context).unwrap()));
    }
    if start.is_some() & end.is_some() && start > end {
            let mut context = Context::new();
            context.insert(
                "err", &"Please enter proper dates..!".to_string()
            );
            return Err(Html(templates.render("matching_days", &context).unwrap()));
    }
    let st_val = f.start.as_deref().unwrap_or("");
    let en_val = f.end.as_deref().unwrap_or("");
    let token  = st_val.to_owned() + "," + en_val;

    Ok(Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", "/booking/search-days")
        .header(
            "Set-Cookie",
            format!(
                "{}={}; Path={}; HttpOnly={}; SameSite={}",
                "period", token, "/", "true", "lax",
            ),
        )
        .body(Body::from("not found"))
        .unwrap())
}

pub async fn post_matching_hours(
    Extension(templates): Extension<Templates>,
    Form(f): Form<FormSearchPrd>
) -> impl IntoResponse {

    let start: Option<NaiveDateTime> = match f.start {
        Some(ref expr) => match Some(NaiveDateTime::parse_from_str(
                    expr.as_ref(), "%Y-%m-%dT%H:%M"
                )
            )
        {
            Some(Ok(res)) => Some(res),
            Some(Err(_)) => None,
            None => None
        }
        None => None
    };
    let end: Option<NaiveDateTime> = match f.end {
        Some(ref expr) => match Some(NaiveDateTime::parse_from_str(
                    expr.as_ref(), "%Y-%m-%dT%H:%M"
                )
            )
        {
            Some(Ok(res)) => Some(res),
            Some(Err(_)) => None,
            None => None
        }
        None => None
    };

    let utc = Some(Utc::now().naive_utc());

    if start.is_some() & end.is_none() && start < utc {
        let mut context = Context::new();
        context.insert(
            "err", &"Please enter proper hours..!".to_string()
        );
        return Err(Html(templates.render("matching_hours", &context).unwrap()));
    }
    if start.is_none() & end.is_some() && end < utc {
        let mut context = Context::new();
        context.insert(
            "err", &"Please enter proper hours..!".to_string()
        );
        return Err(Html(templates.render("matching_hours", &context).unwrap()));
    }
    if start.is_some() & end.is_some() && start > end {
            let mut context = Context::new();
            context.insert(
                "err", &"Please enter proper hours..!".to_string()
            );
            return Err(Html(templates.render("matching_hours", &context).unwrap()));
    }
    let st_val = f.start.as_deref().unwrap_or("");
    let en_val = f.end.as_deref().unwrap_or("");
    let token  = st_val.to_owned() + "," + en_val;

    Ok(Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", "/booking/search-hours")
        .header(
            "Set-Cookie",
            format!(
                "{}={}; Path={}; HttpOnly={}; SameSite={}",
                "period", token, "/", "true", "lax",
            ),
        )
        .body(Body::from("not found"))
        .unwrap())
}

pub async fn post_search_days(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
    Form(f): Form<FormCreatBkgD>,
) -> impl IntoResponse {

    let mut context = Context::new();

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            context.insert("err", &err);
            return Err(Html(templates.render("search_days", &context).unwrap()))
        }
        Ok(None) | Err(None) => {
            context.insert("is_no", "Caramba bullfighting and damn it");
            return Err(Html(templates.render("search_days", &context).unwrap()))
        }
    };

    let number = match f.prv_id.parse::<i32>().map_err(|_| ParsePointError) {
        Ok(expr) => expr,
        Err(err) => {
            context.insert("err", &err);
            return Err(Html(templates.render("search_days", &context).unwrap()))
        }
    };

    let start = Some(f.s_dates);
    let end   = Some(f.e_dates);

    if check_period_days(i.pool.clone(), start, end, number).await.unwrap() {
        let b = BkgPrD {
            user_id: t.id,
            provision_d_id: Some(number),
            title: f.title.to_owned(),
            description: f.description.to_owned(),
            st_date: start,
            en_date: end,
        };
        let _ = creat_bkg_days(i.pool.clone(), b).await.unwrap();
    } else {
        context.insert(
            "err", &"the correspondence of the entered period does not match the date attachments"
        );
        return Err(Html(templates.render("search_days", &context).unwrap()))
    }

    let s_val = f.s_dates;
    let e_val = f.e_dates;

    let zero_date = NaiveDate::parse_from_str("0001-01-01", "%Y-%m-%d").expect("msg");

    if start.is_none() & end.is_none() {
        let _ = Redirect::to("/booking/all-booking").into_response();
    }

    let pg = match i.pool.get().await{
        Ok(expr) => expr,
        Err(err) => {
            context.insert("err", &err.to_string());
            return Err(Html(templates.render("search_days", &context).unwrap()))
        }
    };

    if start.is_some() & end.is_some() {
        let s_vec = vec![s_val];
        let e_vec = vec![e_val];
        let d_vec = vec![s_val, e_val];
        let result = pg.execute(
            "UPDATE provision_d SET s_dates=ARRAY_CAT(s_dates, $2), e_dates=ARRAY_CAT(e_dates, $3), dates=ARRAY_CAT(dates, $4), updated_at=now() WHERE id=$1",
            &[&number, &s_vec, &e_vec, &d_vec]
        ).await;
        let _ = match result {
            Ok(result) => Ok(result),
            Err(err) => Err(err.to_string()),
        };
    }
    if start.is_some() & end.is_none() {
        let s_vec = vec![s_val];
        let e_vec = vec![zero_date];
        let d_vec = vec![s_val, zero_date];
        let result = pg.execute(
            "UPDATE provision_d SET s_dates=ARRAY_CAT(s_dates, $2), e_dates=ARRAY_CAT(e_dates, $3), dates=ARRAY_CAT(dates, $4), updated_at=now() WHERE id=$1",
            &[&number, &s_vec, &e_vec, &d_vec]
        ).await;
        let _ = match result {
            Ok(result) => Ok(result),
            Err(err) => Err(err.to_string()),
        };
    }
    if start.is_none() & end.is_some() {
        let s_vec = vec![zero_date];
        let e_vec = vec![e_val];
        let d_vec = vec![zero_date, e_val];
        let result = pg.execute(
            "UPDATE provision_d SET s_dates=ARRAY_CAT(s_dates, $2), e_dates=ARRAY_CAT(e_dates, $3), dates=ARRAY_CAT(dates, $4), updated_at=now() WHERE id=$1",
            &[&number, &s_vec, &e_vec, &d_vec]
        ).await;
        let _ = match result {
            Ok(result) => Ok(result),
            Err(err) => Err(err.to_string()),
        };
    }

    Ok(Redirect::to("/booking/all-booking").into_response())
}

pub async fn post_search_hours(
    headers: HeaderMap,
    Path(prv_id): Path<String>,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
    Form(f): Form<FormCreatBkgH>,
) -> impl IntoResponse {

    let mut context = Context::new();

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            context.insert("err", &err);
            return Err(Html(templates.render("search_hours", &context).unwrap()))
        }
        Ok(None) | Err(None) => {
            context.insert("is_no", "Caramba bullfighting and damn it");
            return Err(Html(templates.render("search_hours", &context).unwrap()))
        }
    };

    let number = match prv_id.parse::<i32>().map_err(|_| ParsePointError) {
        Ok(expr) => expr,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err", &err);
            return Err(Html(templates.render("search_hours", &context).unwrap()))
        }
    };

    let start = Some(f.s_hours);
    let end   = Some(f.e_hours);

    if check_period_hours(i.pool.clone(), start, end, number).await.unwrap() {
        let b = BkgPrH {
            user_id: t.id,
            provision_h_id: Some(number),
            title: f.title.to_owned(),
            description: f.description.to_owned(),
            st_hour: start,
            en_hour: end,
        };
        let _ = creat_bkg_hours(i.pool.clone(), b).await.unwrap();
    } else {
        context.insert(
            "err", &"the correspondence of the entered period does not match the date attachments"
        );
        return Err(Html(templates.render("detail_hours", &context).unwrap()))
    }

    let s_val = f.s_hours;
    let e_val = f.e_hours;

    let zero_date = NaiveDateTime::parse_from_str("0001-01-01", "%Y-%m-%dT%H:%M").expect("msg");

    if start.is_none() & end.is_none() {
        let _ = Redirect::to("/booking/all-booking").into_response();
    }

    let pg = match i.pool.get().await{
        Ok(expr) => expr,
        Err(err) => {
            context.insert("err", &err.to_string());
            return Err(Html(templates.render("detail_hours", &context).unwrap()))
        }
    };

    if start.is_some() & end.is_some() {
        let s_vec = vec![s_val];
        let e_vec = vec![e_val];
        let d_vec = vec![s_val, e_val];
        let result = pg.execute(
            "UPDATE provision_h SET s_hours=ARRAY_CAT(s_hours, $2), e_hours=ARRAY_CAT(e_hours, $3), hours=ARRAY_CAT(hours, $4), updated_at=now() WHERE id=$1",
            &[&number, &s_vec, &e_vec, &d_vec]
        ).await;
        let _ = match result {
            Ok(result) => Ok(result),
            Err(err) => Err(err.to_string()),
        };
    }
    if start.is_some() & end.is_none() {
        let s_vec = vec![s_val];
        let e_vec = vec![zero_date];
        let d_vec = vec![s_val, zero_date];
        let result = pg.execute(
            "UPDATE provision_h SET s_hours=ARRAY_CAT(s_hours, $2), e_hours=ARRAY_CAT(e_hours, $3), hours=ARRAY_CAT(hours, $4), updated_at=now() WHERE id=$1",
            &[&number, &s_vec, &e_vec, &d_vec]
        ).await;
        let _ = match result {
            Ok(result) => Ok(result),
            Err(err) => Err(err.to_string()),
        };
    }
    if start.is_none() & end.is_some() {
        let s_vec = vec![zero_date];
        let e_vec = vec![e_val];
        let d_vec = vec![zero_date, e_val];
        let result = pg.execute(
            "UPDATE provision_h SET s_hours=ARRAY_CAT(s_hours, $2), e_hours=ARRAY_CAT(e_dates, $3), hours=ARRAY_CAT(hours, $4), updated_at=now() WHERE id=$1",
            &[&number, &s_vec, &e_vec, &d_vec]
        ).await;
        let _ = match result {
            Ok(result) => Ok(result),
            Err(err) => Err(err.to_string()),
        };
    }

    Ok(Redirect::to("/booking/all-booking").into_response())
}