use sqlx::postgres::PgPool;

use axum::{
    extract::{Form, State},
    response::{Html, IntoResponse, Redirect},
    Extension,
};
use chrono::Utc;

use tera::Context;

use axum_extra::TypedHeader;
use headers::Cookie;

use crate::{
    auth,
    common::{DatabaseConn, Templates},
    schedule::models::{FormPlaces, FormSelect, Tickets},
    schedule::views::{all_rec, all_sch, details, places_select, sch_select},
};

pub async fn get_all_sch(
    State(pool): State<PgPool>,
    Extension(templates): Extension<Templates>,
) -> impl IntoResponse {
    let all = all_sch(pool).await.unwrap();

    let mut context = Context::new();
    context.insert("all", &all);
    Html(templates.render("all_sch", &context).unwrap())
}

pub async fn get_all_recording(
    State(pool): State<PgPool>,
    Extension(templates): Extension<Templates>,
) -> impl IntoResponse {
    let all = all_rec(pool).await.unwrap();

    let mut context = Context::new();
    context.insert("all", &all);
    Html(templates.render("all_recording", &context).unwrap())
}

pub async fn get_select(
    State(pool): State<PgPool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let token = auth::views::request_user(cookie).await;
    let _ = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(_) => return Err(Redirect::to("/account/login").into_response()),
    };
    let all = sch_select(pool).await.unwrap();

    let mut context = Context::new();
    context.insert("all", &all);
    Ok(Html(templates.render("select", &context).unwrap()))
}

pub async fn post_select(
    State(pool): State<PgPool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Form(form): Form<FormSelect>,
) -> impl IntoResponse {
    let token = auth::views::request_token(cookie).await.unwrap();
    let owner = &token.claims.id;

    let to_schedule = form.to_schedule;
    let record_d = form.record_d;
    let record_h = form.record_h;

    let _ = sqlx::query(
        "INSERT INTO recording (user_id, to_schedule, record_d, record_h, created_at) VALUES ($1,$2,$3,$4,$5)"
        )
        .bind(owner)
        .bind(to_schedule)
        .bind(record_d)
        .bind(record_h)
        .bind(Utc::now())
        .execute(&pool)
        .await
        .unwrap();

    let occupied = vec![record_h];
    let result = sqlx::query!(
        "UPDATE schedule SET occupied=ARRAY_CAT(occupied, $2), completed=$3, updated_at=$4 WHERE id=$1",
        to_schedule, &occupied, false, Some(Utc::now())
    )
    .fetch_one(&pool).await;

    let _ = match result {
        Ok(result) => Ok(result),
        Err(err) => Err(err.to_string()),
    };
    Redirect::to("/schedule/all-sch").into_response()
}

pub async fn get_places(
    State(pool): State<PgPool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let token = auth::views::request_user(cookie).await;
    let _ = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(_) => return Err(Redirect::to("/account/login").into_response()),
    };
    let all = places_select(pool).await.unwrap();

    let mut context = Context::new();
    context.insert("all", &all);
    Ok(Html(templates.render("places", &context).unwrap()))
}

pub async fn post_places(
    DatabaseConn(mut conn): DatabaseConn,
    TypedHeader(cookie): TypedHeader<Cookie>,
    axum_extra::extract::Form(form): axum_extra::extract::Form<FormPlaces>,
) -> impl IntoResponse {

    let token = auth::views::request_token(cookie).await.unwrap();
    let owner = &token.claims.id;

    let to_schedule = form.to_schedule;
    let record_h = form.record_h;
    let on_off = form.on_off;
    let places = form.places;

    let mut f: Vec<i32> = vec![];
    let mut e = vec![];

    for i in on_off {
        let g = i.parse::<i32>().unwrap();
        f.push(g);
    }
    for (c, d) in f.iter().zip(places.iter()) {
        if *c == 1 {
            e.push(*d);
        }
    }

    let title = details(&mut conn, to_schedule).await.unwrap();
    let t: Tickets = Tickets {
        to_schedule,
        title,
        record_h,
        places,
    };
    let str_t = serde_json::to_string(&t).unwrap();
    let tickets: serde_json::Value = serde_json::from_str(&str_t).unwrap();

    let _ = sqlx::query(
        "INSERT INTO recording (user_id, to_schedule, record_h, places, tickets, created_at) VALUES ($1,$2,$3,$4,$5,$6)"
        )
        .bind(owner)
        .bind(to_schedule)
        .bind(record_h)
        .bind(&e)
        .bind(tickets)
        .bind(Utc::now())
        .execute(&mut *conn)
        .await
        .unwrap();

    let result = sqlx::query!(
        "UPDATE schedule SET non_places=ARRAY_CAT(non_places, $2), completed=$3, updated_at=$4 WHERE id=$1",
        to_schedule, &e, false, Some(Utc::now())
    )
    .fetch_one(&mut *conn).await;

    let _ = match result {
        Ok(result) => Ok(result),
        Err(err) => Err(err.to_string()),
    };
    Redirect::to("/schedule/all-recording").into_response()
}
