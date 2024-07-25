use sqlx::postgres::PgPool;

use axum::{
    extract::{
        Form,
        // Path,
        State
    },
    response::{
        Html,
        IntoResponse,
        Redirect
    },
    Extension,
};
use chrono::{Utc};

use tera::Context;

use axum_extra::TypedHeader;
use headers::Cookie;

use crate::{auth};
use crate::{
    common::{Templates},
    schedule::models::{FormSelect},
    schedule::views::{all_sch, sch_select},
};


pub async fn get_all(
    State(pool): State<PgPool>,
    Extension(templates): Extension<Templates>,
) -> impl IntoResponse {

    let all = all_sch(pool).await.unwrap();

    let mut context = Context::new();
    context.insert("all", &all);
    Html(templates.render("all", &context).unwrap())
}


pub async fn get_select(
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

    let token = auth::views::request_token(TypedHeader(cookie)).await;
    let owner = token.clone().unwrap().claims.id;
    
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
        "UPDATE schedule SET occupied=array_cat(occupied, $2), completed=$3, updated_at=$4 WHERE id=$1", to_schedule, &occupied, false, Some(Utc::now())
        )
        .fetch_one(&pool).await;
    let _ = match result {
        Ok(result) => Ok(result),
        Err(err) => Err(err.to_string()),
    };
    Redirect::to("/recording/all").into_response()
}
