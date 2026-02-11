use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::{State},
    response::{Html, IntoResponse, Redirect},
    Extension,
};
use axum_extra::TypedHeader;
use headers::Cookie;

use futures::{SinkExt, StreamExt};

use std::sync::{Arc};

use tera::Context;

use crate::{
    auth,
    common::Templates,
    chats::models::{UserChat, Msg, InOut},
    chats::repository::{all_public},
    chats::views::{
        insert_joined, insert_message, insert_came_out,
    },
};


fn check_username(
    state: &UserChat, string: &mut String, name: &str
) {

    let mut user_set = state.user_set.lock().unwrap();
    if !user_set.contains(name) {
        user_set.insert(name.to_owned());
        string.push_str(name);
    }
}

async fn ws_handler(
    stream: WebSocket,
    state: Arc<UserChat>,
    cookie: Cookie,
) {

    let token = auth::views::request_token(cookie)
        .await
        .unwrap();
    let i = token.claims.id;

    let (mut sender, mut receiver) = stream.split();
    let mut username = String::new();

    while let Some(Ok(message)) = receiver.next().await {

        if let Message::Text(name) = message {
            check_username(&state, &mut username, &name);
            if !username.is_empty() {
                break;
            } else {
                let _ = sender
                    .send(Message::Text(String::from("Username already taken.").into()))
                    .await;
                return;
            }
        }
    }

    let mut rx = state.tx.subscribe();
    let mut conn = state.pool.acquire().await.unwrap();

    // joined..
    let t: Msg = Msg {
        id: i.to_string(),
        txt: format!("{username} joined.."),
    };
    let s: String = serde_json::to_string(&t).unwrap();
    let _ = state.tx.send(s);
    let _ = insert_joined(&mut conn, token.claims.id, Some(t.txt)).await;

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    // text..
    let tx = state.tx.clone();
    let name = username.clone();
    let mut msg_conn = state.pool.acquire().await.unwrap();

    let mut recv_task = tokio::spawn(async move {

        while let Some(Ok(Message::Text(text))) = receiver.next().await {

            let t: Msg = Msg {
                id: i.to_string(),
                txt: format!("{name}: {text}"),
            };
            let s: String = serde_json::to_string(&t).unwrap();
            let _ = tx.send(s);
            let _ = insert_message(&mut msg_conn, token.claims.id, Some(t.txt)).await;
        }
    });

    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    };

    // out..
    let t: InOut = InOut {txt: format!("{} left.", username)};
    let s: String = serde_json::to_string(&t).unwrap();
    let _ = state.tx.send(s);
    let _ = insert_came_out(&mut conn, token.claims.id, Some(t.txt)).await;

    state.user_set.lock().unwrap().remove(&username);
}


pub async fn us_router(
    ws: WebSocketUpgrade,
    TypedHeader(cookie): TypedHeader<Cookie>,
    State(state): State<Arc<UserChat>>
) -> impl IntoResponse {

    ws.on_upgrade(|socket| ws_handler(socket, state, cookie))
}


pub async fn chat_user(
    State(state): State<Arc<UserChat>>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let token = auth::views::request_user(cookie).await;
    let cls = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(_) => return Err(Redirect::to("/account/login").into_response()),
    };

    let mut conn = state.pool.acquire().await.unwrap();
    let all = all_public(&mut conn).await.unwrap();

    let mut context = Context::new();
    context.insert("id", &cls.id);
    context.insert("name", &cls.username);
    context.insert("all", &all);
    Ok(Html(templates.render("user", &context).unwrap()))
}
