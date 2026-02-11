use std::sync::Arc;

use axum::{routing::get, Extension, Router};

use tera::Tera;

use crate::{
    chats::handler_rm::{rm_router, chat_room},
    chats::models::{RoomChat},
};


pub fn build_routes(chat_state: Arc<RoomChat>) -> Router {

    let mut chat_tera = Tera::default();
    chat_tera
        .add_raw_templates(vec![
            ("base.html", include_str!("../../templates/base.html")),
            ("navbar.html", include_str!("../../templates/navbar.html")),
            (
                "room",
                include_str!("../../templates/chats/room.html"),
            )
        ])
        .unwrap();

    let chats_routes = Router::new().without_v07_checks().nest(
        "/chat-room",
        Router::new()
            // ws
            .without_v07_checks()
            .route("/rm/:c_int", get(rm_router))
            .route("/room/:c_int", get(chat_room))
            .layer(Extension(Arc::new(chat_tera))),
    );
    Router::new().without_v07_checks().merge(chats_routes.with_state(chat_state))
}
