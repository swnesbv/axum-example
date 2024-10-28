use std::sync::Arc;

use axum::{routing::get, Extension, Router};

use tera::Tera;

use crate::{
    chats,
    chats::models::{UserChat},
};


pub fn build_routes(chat_state: Arc<UserChat>) -> Router {

    let mut chat_tera = Tera::default();
    chat_tera
        .add_raw_templates(vec![
            ("base.html", include_str!("../../templates/base.html")),
            ("navbar.html", include_str!("../../templates/navbar.html")),
            (
                "user",
                include_str!("../../templates/chats/user.html"),
            ),
            (
                "dialogue_owner",
                include_str!("../../templates/chats/dialogue_owner.html"),
            ),
        ])
        .unwrap();

    let chats_routes = Router::new().nest(
        "/chat-user",
        Router::new()
            // ws
            .route("/us", get(chats::handler_us::us_router))
            .route("/user", get(chats::handler_us::chat_user))
            .route(
                "/dialogue-owner",
                get(chats::handlers::get_dialogue_owner).post(chats::handlers::post_del_dialogue)
            )
            .route(
                "/del_dialogue",
                get(chats::handlers::get_deletion_dialogue)
            )
            .layer(Extension(Arc::new(chat_tera))),
    );
    Router::new().nest("/", chats_routes.with_state(chat_state))
}
