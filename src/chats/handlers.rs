use axum::{
    extract::{State, Path, Query, OriginalUri},
    response::{Html, IntoResponse, Redirect},
    http::{header::{HeaderMap}},
    Extension,
};
use std::sync::Arc;
use tera::Context;

use crate::{
    common::Templates,
    pgnation::Paginate,
    auth::views::{read_msg},
    chats::models::{UserChat, FormDel, GetParam},
    chats::repository::{
       total_dialogue, user_id_dialogue, vec_del_dialogue, del_dialogue
    },
    photo::repository::{del_msg}
};


pub async fn get_dialogue_owner(
    headers: HeaderMap,
    Query(params): Query<GetParam>,
    State(i): State<Arc<UserChat>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let t = match i.ctx(headers.clone()).await {
        Ok(Some(expr)) => expr,
        Ok(None) | Err(Some(_)) => return Err(Redirect::to("/account/login").into_response()),
        Err(None) => return Err(Redirect::to("/account/login").into_response()),
    };

    let total = total_dialogue(i.pool.clone(), t.id).await;

    let param = match params.page {
        Some(expr) => expr,
        None => return Err(Redirect::to("/chat-user/dialogue-owner?page=1").into_response())
    };
    let page: i64 = param.parse().unwrap();
    let p = Paginate::new(page, 5, 5, total);

    let all = user_id_dialogue(
        i.pool.clone(), t.id, p.p.per_page, p.offset
    ).await.unwrap();

    let msg = read_msg(headers).await.unwrap();

    let mut context = Context::new();

    context.insert("msg", &msg);
    context.insert("cls", &t.id);
    context.insert("all", &all);
    context.insert("p", &p);
    context.insert("i", &t);
    Ok(Html(templates.render("dialogue_owner", &context).unwrap()))
}


pub async fn get_del_dialogue(
    headers: HeaderMap,
    Path(p_int): Path<String>,
    State(i): State<Arc<UserChat>>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Ok(None) | Err(Some(_)) => return Err(Redirect::to("/account/login").into_response()),
        Err(None) => return Err(Redirect::to("/account/login").into_response()),
    };

    let id_i: i32 = p_int.parse().unwrap();
    let _ = del_dialogue(i.pool.clone(), id_i, t.id).await;

    Ok(Redirect::to("/").into_response())
}


pub async fn post_del_dialogue(
    headers: HeaderMap,
    State(i): State<Arc<UserChat>>,
    OriginalUri(original_uri): OriginalUri,
    axum_extra::extract::Form(form): axum_extra::extract::Form<FormDel>
) -> impl IntoResponse {

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Ok(None) | Err(Some(_)) => return Err(Redirect::to("/account/login").into_response()),
        Err(None) => return Err(Redirect::to("/account/login").into_response()),
    };

    let on_off = form.on_off;
    let to_del = form.to_del;

    let mut f: Vec<i32> = vec![];
    let mut e = vec![];

    on_off.iter().for_each(|x| {
        let g = x.parse::<i32>().unwrap();
        f.push(g);
    });
    for (c, d) in f.iter().zip(to_del.iter()) {
        if *c == 1 {
            e.push(*d);
        }
    }

    let _ = vec_del_dialogue(i.pool.clone(), e.clone(), t.id).await;

    Ok({
        del_msg(
            format!("DELETE number: {:?} has been deleted.", e),"danger".to_string(), original_uri.to_string()
        )
    })
}

