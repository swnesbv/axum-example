use std::sync::Arc;

use axum::{routing::get, Extension, Router};

use tera::Tera;

use crate::{
    profile,
    auth::models::{AuthRedis},
};

pub fn rt(state: Arc<AuthRedis>) -> Router {
    let mut base_tera = Tera::default();
    base_tera
        .add_raw_templates(vec![
            ("base.html", include_str!("../../tps/base.html")),
            ("navbar.html", include_str!("../../tps/navbar.html")),
            (
                "rq_user.html",
                include_str!("../../tps/element/rq_user.html")
            ),
            ("index", include_str!("../../tps/index.html")),
        ])
        .unwrap();

    let index_routes = Router::new()
        .route("/", get(profile::handlers::index))
        .layer(Extension(Arc::new(base_tera)));
    Router::new().merge(index_routes.with_state(state))
}
