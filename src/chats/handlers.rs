use axum::{
    extract::{State, Path, Query},
    response::{Html, IntoResponse, Redirect},
    http::{header::{HeaderMap}},
    Extension,
};
use std::sync::Arc;
use tera::Context;

use crate::{
    chats::models::{UserChat, FormDel, GetParam},
    chats::repository::{
       total_dialogue, user_id_dialogue, vec_del_dialogue, del_dialogue
    },
    common::Templates,
    pgnation::Paginate,
};


pub async fn get_dialogue_owner(
    headers: HeaderMap,
    Query(params): Query<GetParam>,
    State(state): State<Arc<UserChat>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let i = match state.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Ok(None) | Err(Some(_)) => return Err(Redirect::to("/account/login").into_response()),
        Err(None) => return Err(Redirect::to("/account/login").into_response()),
    };

    let total = total_dialogue(state.pool.clone(), i.id).await;

    let param = match params.page {
        Some(expr) => expr,
        None => return Err(Redirect::to("/chat-user/dialogue-owner?page=1").into_response())
    };
    let page: i64 = param.parse().unwrap();
    let p = Paginate::new(page, 5, 5, total);

    let all = user_id_dialogue(
        state.pool.clone(), i.id, p.p.per_page, p.offset
    ).await.unwrap();

    let mut context = Context::new();
    context.insert("i", &i);
    context.insert("cls", &i.id);
    context.insert("all", &all);
    context.insert("p", &p);
    Ok(Html(templates.render("dialogue_owner", &context).unwrap()))
}


pub async fn get_del_dialogue(
    headers: HeaderMap,
    Path(p_int): Path<String>,
    State(state): State<Arc<UserChat>>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let i = match state.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Ok(None) | Err(Some(_)) => return Err(Redirect::to("/account/login").into_response()),
        Err(None) => return Err(Redirect::to("/account/login").into_response()),
    };

    let id_i: i32 = p_int.parse().unwrap();
    let _ = del_dialogue(state.pool.clone(), id_i, i.id).await;

    Ok(Redirect::to("/").into_response())
}

pub async fn post_del_dialogue(
    headers: HeaderMap,
    State(state): State<Arc<UserChat>>,
    axum_extra::extract::Form(form): axum_extra::extract::Form<FormDel>,
) -> impl IntoResponse {

    let i = match state.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Ok(None) | Err(Some(_)) => return Err(Redirect::to("/account/login").into_response()),
        Err(None) => return Err(Redirect::to("/account/login").into_response()),
    };

    let on_off = form.on_off;
    let to_del = form.to_del;
    println!(" on_off.. {:?}", on_off);

    let mut f: Vec<i32> = vec![];
    let mut e = vec![];

    for x in on_off {
        let g = x.parse::<i32>().unwrap();
        f.push(g);
    }
    for (c, d) in f.iter().zip(to_del.iter()) {
        if *c == 1 {
            e.push(*d);
        }
    }
    println!(" e.. {:?}", e);

    let _ = vec_del_dialogue(state.pool.clone(), e, i.id).await;

    Ok(Redirect::to("/chat-user/dialogue-owne").into_response())
}

