use sqlx::postgres::PgPool;
use std::sync::Arc;

use axum::{routing::get, Extension, Router};

use tera::Tera;

use crate::subscriptions;


pub fn build_routes(pool: PgPool) -> Router {
    let mut ssc_tera = Tera::default();
    ssc_tera
        .add_raw_templates(vec![
            ("base.html", include_str!("../../templates/base.html")),
            ("navbar.html", include_str!("../../templates/navbar.html")),
            (
                "creat",
                include_str!("../../templates/subscriptions/creat.html"),
            ),
            (
                "groups",
                include_str!("../../templates/subscriptions/groups.html"),
            ),
            (
                "ssc_owner",
                include_str!("../../templates/subscriptions/ssc_owner.html"),
            ),
            (
                "ssc_to_user",
                include_str!("../../templates/subscriptions/ssc_to_user.html"),
            ),
            (
                "ssc_group",
                include_str!("../../templates/subscriptions/ssc_group.html"),
            ),
        ])
        .unwrap();

    let subscriptions_routes = Router::new().nest(
        "/subscriptions",
        Router::new()
            .route(
                "/creat-group",
                get(subscriptions::creat::get_creat_group)
                .post(subscriptions::creat::post_creat_group)
            )
            .route(
                "/groups",
                get(subscriptions::handlers::get_groups)
            )
            .route(
                "/ssc-owner",
                get(subscriptions::handlers::get_owner_ssc)
            )
            .route(
                "/ssc-to-user",
                get(subscriptions::handlers::get_to_user)
                .post(subscriptions::handlers::post_resolution_user)
            )
            .layer(Extension(Arc::new(ssc_tera))),
    );
    Router::new().nest("/", subscriptions_routes.with_state(pool))
}
