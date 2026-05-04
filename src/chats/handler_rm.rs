use std::sync::{Arc};
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::{State, Path},
    response::{Html, IntoResponse, Redirect},
    http::{header::{HeaderMap}},
    Extension,
};
use futures::{SinkExt, StreamExt};
use tokio::sync::broadcast;
use tera::Context;

use crate::{
    common::Templates,
    chats::models::{RoomChat, DialogueChat, RoomState, Connect, Msg, InOut},
    chats::repository::{to_room, to_dialogue, ssc_dialogue},
    chats::views::{
        dialogue_joined, insert_msg_room, dialogue_came_out,
    },
};

pub async fn ws_handler(
    c_int: String,
    headers: HeaderMap,
    stream: WebSocket,
    state: Arc<RoomChat>,
) {

    let i = match state.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Ok(None) | Err(Some(_)) => return,
        Err(None) => return,
    };

    let (mut sender, mut receiver) = stream.split();
    let mut tx = None::<broadcast::Sender<String>>;
    let mut username = String::new();
    let mut channel = String::new();

    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(name) = message {
            let _ = Connect {
                username: i.username.to_string(),
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

    // joined..
    let t: Msg = Msg {
        id: i.id.to_string(),
        txt: format!("{username} joined.."),
    };
    let s: String = serde_json::to_string(&t).unwrap();
    let _ = tx.send(s);
    let _ = dialogue_joined(state.pool.clone(), i.id, Some(t.txt), c_int.clone()).await;

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.clone().into())).await.is_err() {
                break;
            }
        }
    });

    // text..

    let duplicate = state.clone();
    let mut recv_task = {
        let tx = tx.clone();
        let name = username.clone();
        let clone_c_int = c_int.to_owned();

        tokio::spawn(async move {

            while let Some(Ok(Message::Text(text))) = receiver.next().await {
                let t: Msg = Msg {
                    id: i.id.to_string(),
                    txt: format!("{name}: {text}"),
                };
                let s: String = serde_json::to_string(&t).unwrap();
                let _ = tx.send(s);
                let _ = insert_msg_room(
                    duplicate.pool.clone(), i.id, Some(t.txt), clone_c_int.to_owned()
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
    let _ = dialogue_came_out(state.pool.clone(), i.id, Some(t.txt), c_int).await;

    let mut rooms = state.rooms.lock().unwrap();

    rooms.get_mut(&channel).unwrap().user_set.remove(&username);
}


pub async fn rm_router(
    ws: WebSocketUpgrade,
    Path(c_int): Path<String>,
    headers: HeaderMap,
    State(state): State<Arc<RoomChat>>
) -> impl IntoResponse {

    ws.on_upgrade(|socket| ws_handler(c_int, headers, socket, state))
}

pub async fn chat_room(
    Path(c_int): Path<String>,
    State(state): State<Arc<RoomChat>>,
    headers: HeaderMap,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let i = match state.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Ok(None) | Err(Some(_)) => return Err(Redirect::to("/account/login").into_response()),
        Err(None) => return Err(Redirect::to("/account/login").into_response()),
    };
    let room = ":".to_owned() + &c_int.clone();

    let who: DialogueChat = to_dialogue(state.pool.clone(), c_int.clone()).await;
    let who_who: Option<i32> = who.to_user;
    let find_out = match who_who {
        Some(expr) => expr,
        None => return Err(Redirect::to("/account/login").into_response())
    };

    if i.id == who.user_id || i.id == find_out {
        let all = to_room(
            state.pool.clone(), room.clone()
        ).await.unwrap();
        let ssc = ssc_dialogue(state.pool.clone(), i.id).await.unwrap();

        let mut context = Context::new();
        context.insert("c_int", &room);
        context.insert("id", &i.id);
        context.insert("name", &i.username);
        context.insert("all", &all);
        context.insert("ssc", &ssc);
        Ok(Html(templates.render("room", &context).unwrap()))
    } else {
        Err(Redirect::to("/account/login").into_response())
    }
}
