use std::sync::Arc;
use axum::{
    extract::{Form, State},
    response::{Html, IntoResponse, Redirect},
    http::{header::{HeaderMap}},
    Extension,
};
use tera::Context;

use crate::{
    common::{Templates},
    auth::models::{AuthRedis},
    schedule::models::{FormPlaces, FormSelect, Tickets},
    schedule::views::{all_rec, all_sch, details, places_select, sch_select},
};

pub async fn get_all_sch(
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>
) -> impl IntoResponse {
    let all = all_sch(i.pool.clone()).await.unwrap();

    let mut context = Context::new();
    context.insert("all", &all);
    Html(templates.render("all_sch", &context).unwrap())
}

pub async fn get_all_recording(
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>
) -> impl IntoResponse {
    let all = all_rec(i.pool.clone()).await.unwrap();

    let mut context = Context::new();
    context.insert("all", &all);
    Html(templates.render("all_recording", &context).unwrap())
}

pub async fn get_select(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>
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
            return Err(Html(templates.render("select", &context).unwrap()))
        }
    };

    let all = sch_select(i.pool.clone()).await.unwrap();

    context.insert("t", &t);
    context.insert("all", &all);
    Ok(Html(templates.render("select", &context).unwrap()))
}

pub async fn post_select(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
    Form(f): Form<FormSelect>,
) -> impl IntoResponse {

    let mut context = Context::new();
    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            context.insert("err", &err);
            return Err(Html(templates.render("select", &context).unwrap()))
        }
        Ok(None) | Err(None) => {
            context.insert("err", "Caramba bullfighting and damn it");
            return Err(Html(templates.render("select", &context).unwrap()))
        }
    };

    let to_schedule = f.to_schedule;
    let record_d    = f.record_d;
    let record_h    = f.record_h;

    let pg = match i.pool.get().await{
        Ok(expr) => expr,
        Err(err) => {
            context.insert("err", &err.to_string());
            return Err(Html(templates.render("select", &context).unwrap()))
        }
    };

    let _ = pg.query(
        "INSERT INTO recording (user_id, to_schedule, record_d, record_h, created_at) VALUES ($1,$2,$3,$4,now())",
        &[&t.id, &to_schedule, &record_d, &record_h]
        ).await.unwrap();

    let occupied = vec![record_h];
    let result = pg.query(
        "UPDATE schedule SET occupied=ARRAY_CAT(occupied, $2), completed=$3, updated_at=now() WHERE id=$1",
        &[&to_schedule, &occupied, &false]
    ).await;

    let _ = match result {
        Ok(result) => Ok(result),
        Err(err) => Err(err.to_string()),
    };
    Ok(Redirect::to("/schedule/all-sch").into_response())
}

pub async fn get_places(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            context.insert("err", &err);
            return Err(Html(templates.render("places", &context).unwrap()))
        }
        Ok(None) | Err(None) => {
            context.insert("err", "Caramba bullfighting and damn it");
            return Err(Html(templates.render("places", &context).unwrap()))
        }
    };

    let all = places_select(i.pool.clone()).await.unwrap();

    context.insert("t", &t);
    context.insert("all", &all);
    Ok(Html(templates.render("places", &context).unwrap()))
}

pub async fn post_places(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
    axum_extra::extract::Form(f): axum_extra::extract::Form<FormPlaces>,
) -> impl IntoResponse {

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            let mut context = Context::new();
            context.insert("err", &err);
            return Err(Html(templates.render("places", &context).unwrap()))
        }
        Ok(None) | Err(None) => {
            let mut context = Context::new();
            context.insert("err", "Caramba bullfighting and damn it");
            return Err(Html(templates.render("places", &context).unwrap()))
        }
    };

    let to_schedule = f.to_schedule;
    let record_h    = f.record_h;
    let on_off      = f.on_off;
    let places      = f.places;

    let mut v: Vec<i32> = vec![];
    let mut e = vec![];

    for i in on_off {
        let g = i.parse::<i32>().unwrap();
        v.push(g);
    }
    for (c, d) in v.iter().zip(places.iter()) {
        if *c == 1 {
            e.push(*d);
        }
    }

    let title = details(i.pool.clone(), to_schedule).await.unwrap();
    let sch: Tickets = Tickets {
        to_schedule,
        title,
        record_h,
        places,
    };
    let str_t = serde_json::to_string(&sch).unwrap();
    let tickets: serde_json::Value = serde_json::from_str(&str_t).unwrap();

    let pg = match i.pool.get().await{
        Ok(expr) => expr,
        Err(err) => {
            let mut context = Context::new();
            context.insert("err", &err.to_string());
            return Err(Html(templates.render("places", &context).unwrap()))
        }
    };

    let _ = pg.query(
        "INSERT INTO recording (user_id, to_schedule, record_h, places, tickets, created_at) VALUES ($1,$2,$3,$4,$5,now())",
        &[&t.id, &to_schedule, &record_h, &e, &tickets]
    ).await.unwrap();

    let result = pg.query(
        "UPDATE schedule SET non_places=ARRAY_CAT(non_places, $2), completed=$3, updated_at=$4 WHERE id=$1",
        &[&to_schedule, &e, &false]
    ).await;

    let _ = match result {
        Ok(result) => Ok(result),
        Err(err) => Err(err.to_string()),
    };
    Ok(Redirect::to("/schedule/all-recording").into_response())
}
