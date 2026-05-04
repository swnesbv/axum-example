use std::sync::Arc;
use axum::{routing::get, Extension, Router};
use tera::Tera;

use crate::{
    photo::handlers,
    auth::models::{AuthRedis},
};

pub fn rt(state: Arc<AuthRedis>) -> Router {
    let mut photo_tera = Tera::default();
    photo_tera
        .add_raw_templates(vec![
            ("base.html", include_str!("../../tps/base.html")),
            ("navbar.html", include_str!("../../tps/navbar.html")),
            (
                "rq_user.html",
                include_str!("../../tps/element/rq_user.html")
            ),
            (
                "photo",
                include_str!("../../tps/photo/photo.html")
            ),
        ])
        .unwrap();

    let photo_routes = Router::new()
        .route(
            "/photo",
            get(handlers::get_photo_users)
            .post(handlers::photo_users),
        )
        .layer(Extension(Arc::new(photo_tera)));
    Router::new().merge(photo_routes.with_state(state))
}
