use std::sync::{Arc};
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::{State},
    response::{Html, IntoResponse, Redirect},
    http::{header::{HeaderMap}},
    Extension,
};
use futures::{SinkExt, StreamExt};
use tera::Context;

use crate::{
    common::Templates,
    chats::models::{UserChat, Msg, InOut},
    chats::repository::{all_public},
    chats::views::{
        insert_joined, insert_msg_pch, insert_came_out,
    },
};


fn check_username(
    state: &UserChat,
    string: &mut String,
    name: &str
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
    headers: HeaderMap,
) {

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

    let i = match state.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Ok(None) | Err(Some(_)) => return,
        Err(None) => return,
    };

    let mut rx = state.tx.subscribe();

    // joined..
    let t: Msg = Msg {
        id: i.id.to_string(),
        txt: format!("{username} joined.."),
    };
    let s: String = serde_json::to_string(&t).unwrap();
    let _ = state.tx.send(s);
    let _ = insert_joined(state.pool.clone(), i.id, Some(t.txt)).await;

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

    let duplicate = state.clone();
    let mut recv_task = tokio::spawn(async move {

        while let Some(Ok(Message::Text(text))) = receiver.next().await {

            let t: Msg = Msg {
                id: i.id.to_string(),
                txt: format!("{name}: {text}"),
            };
            let s: String = serde_json::to_string(&t).unwrap();
            let _ = tx.send(s);
            let _ = insert_msg_pch(duplicate.pool.clone(), i.id, Some(t.txt)).await;
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
    let _ = insert_came_out(state.pool.clone(), i.id, Some(t.txt)).await;

    state.user_set.lock().unwrap().remove(&username);
}


pub async fn us_router(
    ws: WebSocketUpgrade,
    headers: HeaderMap,
    State(state): State<Arc<UserChat>>
) -> impl IntoResponse {

    ws.on_upgrade(|socket| ws_handler(socket, state, headers))
}


pub async fn chat_user(
    State(state): State<Arc<UserChat>>,
    headers: HeaderMap,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let i = match state.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Ok(None) | Err(Some(_)) => return Err(Redirect::to("/account/login").into_response()),
        Err(None) => return Err(Redirect::to("/account/login").into_response()),
    };

    let all = all_public(state.pool.clone()).await.unwrap();

    let mut context = Context::new();
    context.insert("id", &i.id);
    context.insert("name", &i.username);
    context.insert("all", &all);
    Ok(Html(templates.render("user", &context).unwrap()))
}
