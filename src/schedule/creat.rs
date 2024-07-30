use sqlx::postgres::PgPool;

use std::str;

// use axum_extra::extract::Form;

use axum::{
    extract::{
        // Form, 
        State},
    response::{Html, IntoResponse, Redirect},
    Extension,
};

use chrono::{
    NaiveDateTime,
    Utc
};

use tera::Context;

use axum_extra::TypedHeader;
use headers::Cookie;

use crate::{auth};
use crate::{
    common::{Templates},
    schedule::models::{
        FormSch
    },
    util::q_body::{InputBody}
};


pub async fn get_creat(
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    
    let token = auth::views::request_user(TypedHeader(cookie)).await;
    let _ = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(_) => return Err(Redirect::to("/account/login").into_response()),
    };
    Ok(Html(
        templates.render("creat", &Context::new()).unwrap(),
    ))
}


/*pub async fn post_creat(
    State(pool): State<PgPool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
    Form(form): Form<FormSch>,
) -> impl IntoResponse {

    let token = auth::views::request_token(TypedHeader(cookie)).await;

    println!("form..{:?}", form);

    let s_val = form.st_hour.as_deref().unwrap_or("err..");
    let e_val = form.en_hour.as_deref().unwrap_or("err..");
    let start: Option<NaiveDateTime> = if form.st_hour.is_some() {
        Some(
            NaiveDateTime::parse_from_str(s_val, "%Y-%m-%dT%H:%M").unwrap()
        )
    } else {
        None
    };
    let end: Option<NaiveDateTime> = if form.en_hour.is_some() {
        Some(
            NaiveDateTime::parse_from_str(e_val, "%Y-%m-%dT%H:%M").unwrap()
        )
    } else {
        None
    };

    let l_val = form.list.as_deref().unwrap();
    let mut hours = Some(Vec::new());
    if form.list.is_some() {
        for i in l_val {
            if !i.is_empty() {
                hours.as_mut().expect("REASON").push(NaiveDateTime::parse_from_str(i, "%Y-%m-%dT%H:%M").unwrap())
            }
        }
    } else {
        hours = None
    }

    let result = sqlx::query(
        "INSERT INTO schedule (user_id, title, description, st_hour, en_hour, hours, created_at) VALUES ($1,$2,$3,$4,$5,$6,$7)"
        )
        .bind(token.clone().unwrap().claims.id)
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


pub async fn post_creat(
    State(pool): State<PgPool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
    InputBody(body): InputBody,
) -> impl IntoResponse {

    let form: FormSch = serde_urlencoded::from_str(str::from_utf8(&body).unwrap()).unwrap();
    println!("body..{:?}", body);
    println!("form..{:?}", form);

    let token = auth::views::request_token(TypedHeader(cookie)).await;

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

    let mut hours = Some(Vec::new());
    if form.list.as_ref().expect("REASON").len() > 0 {
        let l_val = form.list.as_deref().unwrap();
        for i in l_val {
            if !i.is_empty() {
                hours.as_mut().expect("REASON").push(NaiveDateTime::parse_from_str(i, "%Y-%m-%dT%H:%M").unwrap())
            }
        }
    } else {
        hours = None
    }

    let result = sqlx::query(
        "INSERT INTO schedule (user_id, title, description, st_hour, en_hour, hours, created_at) VALUES ($1,$2,$3,$4,$5,$6,$7)"
        )
        .bind(token.clone().unwrap().claims.id)
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
}


/*pub async fn post_creat(
    State(pool): State<PgPool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
    Form(form): Form<FormSch>,
) -> impl IntoResponse {

    let token = auth::views::request_token(TypedHeader(cookie)).await;

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
        .bind(token.clone().unwrap().claims.id)
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
