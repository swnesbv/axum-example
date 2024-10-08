use axum::{
    extract::{State, Path, Query},
    response::{Html, IntoResponse, Redirect},
    Extension,
};

use std::sync::Arc;

use tera::Context;

use axum_extra::TypedHeader;
use headers::Cookie;

use crate::{
    auth,
    chats::models::{UserChat, FormDel, GetParam},
    chats::repository::{
       total_dialogue, user_id_dialogue, vec_del_dialogue, del_dialogue
    },
    common::Templates,
    pgnation::Paginate,
};


pub async fn get_dialogue_owner(
    Query(params): Query<GetParam>,
    State(state): State<Arc<UserChat>>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let token = auth::views::request_user(cookie).await;
    let c = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(_) => return Err(Redirect::to("/account/login").into_response()),
    };

    let mut conn = state.pool.acquire().await.unwrap();

    let total = total_dialogue(
        &mut conn, c.id
    ).await;
    let page: i64 = if params.page.is_empty() {
        1
    } else {
        params.page.parse().expect("Not a valid number")
    };
    let p = Paginate::new(page, 5, 5, total);

    let all = user_id_dialogue(
        &mut conn, c.id, p.p.per_page, p.offset
    ).await.unwrap();

    let mut context = Context::new();
    context.insert("cls", &c.id);
    context.insert("all", &all);
    context.insert("p", &p);
    Ok(Html(templates.render("dialogue_owner", &context).unwrap()))
}

pub async fn post_del_dialogue(
    State(state): State<Arc<UserChat>>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    axum_extra::extract::Form(form): axum_extra::extract::Form<FormDel>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let token = auth::views::request_user(cookie).await;
    let cls = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(_) => return Err(Redirect::to("/account/login").into_response()),
    };

    let on_off = form.on_off;
    let to_del = form.to_del;
    println!(" on_off.. {:?}", on_off);

    let mut f: Vec<i32> = vec![];
    let mut e = vec![];

    for i in on_off {
        let g = i.parse::<i32>().unwrap();
        f.push(g);
    }
    for (c, d) in f.iter().zip(to_del.iter()) {
        if *c == 1 {
            e.push(*d);
        }
    }
    println!(" e.. {:?}", e);

    let mut conn = state.pool.acquire().await.unwrap();
    let _ = vec_del_dialogue(&mut conn, e, cls.id).await;

    Ok(Redirect::to("/").into_response())
}


pub async fn get_deletion_dialogue(
    Path(p_int): Path<String>,
    State(state): State<Arc<UserChat>>,
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let token = auth::views::request_user(cookie).await;
    let cls = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(_) => return Err(Redirect::to("/account/login").into_response()),
    };

    let id_i: i32 = p_int.parse().unwrap();
    let mut conn = state.pool.acquire().await.unwrap();
    let _ = del_dialogue(&mut conn, id_i, cls.id).await;

    Ok(Redirect::to("/").into_response())
}
