use std::sync::Arc;
use axum::{
    extract::{State},
    response::{Html, IntoResponse, Redirect},
    http::{header::{HeaderMap}},
    Extension,
};
use chrono::{NaiveDateTime};
use tera::Context;

use crate::{
    common::Templates,
    auth::models::{AuthRedis},
    schedule::models::FormSch,
    // util::r_body::InputBody
};

pub async fn get_creat(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            context.insert("err", &err.to_string());
            return Ok(Html(templates.render("creat", &context).unwrap()));
        }
        Ok(None) | Err(None) => return Err(Redirect::to("/account/login").into_response()),
    };

    context.insert("t", &t);
    Ok(Html(templates.render("creat", &context).unwrap()))
}

#[axum::debug_handler()]
pub async fn post_creat(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
    axum_extra::extract::Form(f): axum_extra::extract::Form<FormSch>,
) -> impl IntoResponse {

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            let mut context = Context::new();
            context.insert("err", &err.to_string());
            return Err(Html(templates.render("creat", &context).unwrap()));
        }
        Ok(None) | Err(None) => return Ok(Redirect::to("/account/login").into_response()),
    };



    let start: Option<NaiveDateTime> = f.st_hour.map(|expr| NaiveDateTime::parse_from_str(&expr, "%Y-%m-%dT%H:%M").unwrap());
    let end: Option<NaiveDateTime> = f.en_hour.map(|expr| NaiveDateTime::parse_from_str(&expr, "%Y-%m-%dT%H:%M").unwrap());

    let mut hours: Option<Vec<NaiveDateTime>> = Some(Vec::new());
    hours = match f.list {
        Some(expr) => {
            for i in expr {
                if !i.is_empty() {
                    hours.as_mut().expect("REASON").push(NaiveDateTime::parse_from_str(&i, "%Y-%m-%dT%H:%M").unwrap())
                }
            }
            None
        }
        None => None
    };

    let pg = match i.pool.get().await{
        Ok(expr) => expr,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err", &err.to_string());
            return Err(Html(templates.render("creat", &context).unwrap()));
        }
    };
    let result = pg.execute(
        "INSERT INTO schedule (user_id, title, description, st_hour, en_hour, hours, created_at) VALUES ($1,$2,$3,$4,$5,$6,now())",
         &[&t.id, &f.title, &f.description, &start, &end, &hours]
    ).await;
    match result {
        Ok(result) => result,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err_token", &err.to_string());
            return Err(Html(templates.render("creat", &context).unwrap()));
        }
    };
    Ok(Redirect::to("/schedule/all-sch").into_response())
}


/*pub async fn post_creat(
    State(pool): State<PgPool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
    InputBody(body): InputBody,
) -> impl IntoResponse {

    let form: FormSch = serde_urlencoded::from_str(std::str::from_utf8(&body).unwrap()).unwrap();

    let token = auth::views::request_token(cookie).await.unwrap();

    let s_val = form.st_hour.as_deref().unwrap_or("err..");
    let e_val = form.en_hour.as_deref().unwrap_or("err..");

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

    let mut hours = Some(Vec::new());
    if !form.list.as_ref().expect("REASON").is_empty() {
        let l_val = form.list.as_deref().unwrap();
        for i in l_val {
            if !i.is_empty() {
                hours
                    .as_mut()
                    .expect("REASON")
                    .push(NaiveDateTime::parse_from_str(i, "%Y-%m-%dT%H:%M").unwrap())
            }
        }
    } else {
        hours = None
    }

    let mut places = Some(Vec::new());
    if form.places.is_some() {
        for i in 1..=form.places.expect("REASON") {
            places.as_mut().expect("REASON").push(i)
        }
    } else {
        places = None
    }

    let result = sqlx::query(
        "INSERT INTO schedule (user_id, title, description, st_hour, en_hour, hours, places, created_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8)"
        )
        .bind(token.claims.id)
        .bind(&form.title)
        .bind(&form.description)
        .bind(start)
        .bind(end)
        .bind(&hours)
        .bind(&places)
        .bind(Utc::now())
        .execute(&pool)
        .await;
    match result {
        Ok(result) => result,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err_token", &err.to_string());
            return Err(Html(templates.render("creat", &context).unwrap()));
        }
    };
    Ok(Redirect::to("/schedule/all-sch").into_response())
}*/

/*pub async fn post_creat(
    State(pool): State<PgPool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
    Form(form): Form<FormSch>,
) -> impl IntoResponse {

    let token = auth::views::request_token(cookie).await.unwrap();

    let s_val = form.st_hour.as_deref().unwrap_or("err..");
    let e_val = form.en_hour.as_deref().unwrap_or("err..");
    let start: Option<NaiveDateTime> = if !s_val.is_empty() {
        Some(
            NaiveDateTime::parse_from_str(s_val, "%Y-%m-%dT%H:%M").unwrap()
        )
    } else {
        None
    };
    let end: Option<NaiveDateTime> = if !e_val.is_empty() {
        Some(
            NaiveDateTime::parse_from_str(e_val, "%Y-%m-%dT%H:%M").unwrap()
        )
    } else {
        None
    };

    let l_val = form.vec_list.as_deref().unwrap();
    let mut hours = Vec::new();
    if !l_val.is_empty() {
        for i in l_val.split(&[','][..]) {
            hours.push(
                match NaiveDateTime::parse_from_str(i, "%Y-%m-%d %H:%M") {
                    Ok(convert) => convert,
                    Err(err) => {
                        let mut context = Context::new();
                        context.insert("err_token", &err.to_string());
                        return Err(
                            Html(templates.render("creat", &context).unwrap())
                        );
                    }
                }
            )
        };
        Some(
            ()
        )
    } else {
        None
    };

    let result = sqlx::query(
        "INSERT INTO schedule (user_id, title, description, st_hour, en_hour, hours, created_at) VALUES ($1,$2,$3,$4,$5,$6,$7)"
        )
        .bind(&token.claims.id)
        .bind(form.title.clone())
        .bind(form.description.clone())
        .bind(start)
        .bind(end)
        .bind(&hours)
        .bind(Utc::now())
        .execute(&pool)
        .await;
    match result {
        Ok(result) => result,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err_token", &err.to_string());
            return Err(Html(templates.render("creat", &context).unwrap()));
        }
    };
    Ok(Redirect::to("/schedule/all").into_response())
}*/
