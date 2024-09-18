use sqlx::postgres::PgPool;
use std::sync::Arc;

use axum::{routing::get, Extension, Router};

use tera::Tera;

use crate::profile;


pub fn build_routes(pool: PgPool) -> Router {
    let mut base_tera = Tera::default();
    base_tera
        .add_raw_templates(vec![
            ("base.html", include_str!("../templates/base.html")),
            ("navbar.html", include_str!("../templates/navbar.html")),
            ("index", include_str!("../templates/index.html")),
        ])
        .unwrap();

    let index_routes = Router::new().nest(
        "/",
        Router::new()
            .route("/", get(profile::handlers::index))
            .layer(Extension(Arc::new(base_tera))),
    );
    Router::new().nest("/", index_routes.with_state(pool))
}
