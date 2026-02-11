use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::{State, Path},
    response::{Html, IntoResponse, Redirect},
    Extension,
};
use axum_extra::TypedHeader;
use headers::Cookie;

use futures::{SinkExt, StreamExt};

use std::sync::{Arc};

use tokio::sync::broadcast;

use tera::Context;

use crate::{
    auth,
    common::Templates,
    chats::models::{RoomChat, DialogueChat, RoomState, Connect, Msg, InOut},
    chats::repository::{to_room, to_dialogue, ssc_dialogue},
    chats::views::{
        dialogue_joined, dialogue_message, dialogue_came_out,
    },
};


pub async fn ws_handler(
    c_int: String,
    cookie: Cookie,
    stream: WebSocket,
    state: Arc<RoomChat>,
) {

    let i = auth::views::request_token(cookie)
        .await
        .unwrap();
    let c = i.claims.id;

    let (mut sender, mut receiver) = stream.split();

    let mut tx = None::<broadcast::Sender<String>>;
    let mut username = String::new();
    let mut channel = String::new();

    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(name) = message {
            let _ = Connect {
                username: i.claims.username.to_string(),
                channel: c_int.clone(),
            };

            let connect: Connect = match serde_json::from_str(&name) {
                Ok(connect) => connect,
                Err(err) => {
                    println!(" err..{:?}", err);
                    let _ = sender
                        .send(Message::Text(String::from(
                            "Failed to parse connect message",
                        ).into()))
                        .await;
                    break;
                }
            };
            {
                let mut rooms = state.rooms.lock().unwrap();

                channel = connect.channel.clone();
                let room = rooms.entry(connect.channel).or_insert_with(RoomState::new);
                tx = Some(room.tx.clone());

                if !room.user_set.contains(&connect.username) {
                    room.user_set.insert(connect.username.to_owned());
                    username = connect.username.clone();
                }
            }
            if tx.is_some() && !username.is_empty() {
                break;
            } else {
                let _ = sender
                    .send(Message::Text(String::from("Username already taken.").into()))
                    .await;
                return;
            }
        }
    }

    let tx = tx.unwrap();
    let mut rx = tx.subscribe();
    let mut conn = state.pool.acquire().await.unwrap();

    // joined..
    let t: Msg = Msg {
        id: c.to_string(),
        txt: format!("{username} joined.."),
    };
    let s: String = serde_json::to_string(&t).unwrap();
    let _ = tx.send(s);
    let _ = dialogue_joined(&mut conn, c, Some(t.txt), c_int.clone()).await;

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.clone().into())).await.is_err() {
                break;
            }
        }
    });

    // text..
    let mut recv_task = {
        let tx = tx.clone();
        let name = username.clone();
        let clone_c_int = c_int.to_owned();
        let mut msg_conn = state.pool.acquire().await.unwrap();

        tokio::spawn(async move {

            while let Some(Ok(Message::Text(text))) = receiver.next().await {

                let t: Msg = Msg {
                    id: c.to_string(),
                    txt: format!("{name}: {text}"),
                };
                let s: String = serde_json::to_string(&t).unwrap();
                let _ = tx.send(s);
                let _ = dialogue_message(
                    &mut msg_conn, c, Some(t.txt), clone_c_int.to_owned()
                ).await;
            }
        })
    };

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }

    // out..
    let t: InOut = InOut {txt: format!("{} left.", username)};
    let s: String = serde_json::to_string(&t).unwrap();
    let _ = tx.send(s);
    let _ = dialogue_came_out(&mut conn, c, Some(t.txt), c_int).await;

    let mut rooms = state.rooms.lock().unwrap();

    rooms.get_mut(&channel).unwrap().user_set.remove(&username);
}


pub async fn rm_router(
    ws: WebSocketUpgrade,
    Path(c_int): Path<String>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    State(state): State<Arc<RoomChat>>
) -> impl IntoResponse {

    ws.on_upgrade(|socket| ws_handler(c_int, cookie, socket, state))
}

pub async fn chat_room(
    Path(c_int): Path<String>,
    State(state): State<Arc<RoomChat>>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let token = auth::views::request_user(cookie).await;
    let i = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(_) => return Err(Redirect::to("/account/login").into_response()),
    };
    let c = i.id;

    let mut conn = state.pool.acquire().await.unwrap();

    let dialogue: DialogueChat = to_dialogue(&mut conn, c_int.clone()).await;
    let pat: Option<i32> = dialogue.to_user;
    let m = match pat {
        Some(pat) => pat,
        None => pat.expect("REASON"),
    };
    if c == dialogue.user_id || c == m {
        let all = to_room(&mut conn, c_int.clone()).await.unwrap();
        let ssc = ssc_dialogue(&mut conn, c).await.unwrap();

        let mut context = Context::new();
        context.insert("c_int", &c_int);
        context.insert("id", &c);
        context.insert("name", &i.username);
        context.insert("all", &all);
        context.insert("ssc", &ssc);
        return Ok(Html(templates.render("room", &context).unwrap()));
    };
    Ok(Html(templates.render("index", &Context::new()).unwrap()))
}
