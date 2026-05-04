use std::sync::Arc;
use axum::{routing::get, Extension, Router};
use tera::Tera;

use crate::{
    subscriptions,
    auth::models::{AuthRedis},
};

pub fn rt(state: Arc<AuthRedis>) -> Router {
    let mut ssc_tera = Tera::default();
    ssc_tera
        .add_raw_templates(vec![
            ("base.html", include_str!("../../tps/base.html")),
            ("navbar.html", include_str!("../../tps/navbar.html")),
            (
                "rq_user.html",
                include_str!("../../tps/element/rq_user.html")
            ),
            (
                "created_updated.html",
                include_str!("../../tps/element/created_updated.html")
            ),
            (
                "completed.html",
                include_str!("../../tps/element/completed.html")
            ),
            (
                "creat",
                include_str!("../../tps/subscriptions/creat.html"),
            ),
            (
                "groups",
                include_str!("../../tps/subscriptions/groups.html"),
            ),
            (
                "ssc_owner",
                include_str!("../../tps/subscriptions/ssc_owner.html"),
            ),
            (
                "ssc_to_user",
                include_str!("../../tps/subscriptions/ssc_to_user.html"),
            ),
            (
                "ssc_group",
                include_str!("../../tps/subscriptions/ssc_group.html"),
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
    Router::new().merge(subscriptions_routes.with_state(state))
}
