use std::sync::Arc;
use axum::{routing::get, Extension, Router};
use tera::Tera;

use crate::{
    purchases,
    auth::models::{AuthRedis}
};


pub fn rt(state: Arc<AuthRedis>) -> Router {
    let mut purchases_tera = Tera::default();
    purchases_tera
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
                "comments.html",
                include_str!("../../tps/element/comments.html")
            ),
            (
                "completed.html",
                include_str!("../../tps/element/completed.html")
            ),
            (
                "all",
                include_str!("../../tps/purchases/all.html"),
            ),
            ("order", include_str!("../../tps/purchases/order.html")),
            // ("select", include_str!("../../tps/purchases/select.html")),
            // ("categories", include_str!("../../tps/purchases/categories.html")),
            // ("detail", include_str!("../../tps/purchases/detail.html")),
            // ("delete", include_str!("../../tps/schedule/delete.html")),
        ])
        .unwrap();

    let purchases_routes = Router::new().without_v07_checks().nest(
        "/purchases",
        Router::new()
            .without_v07_checks()
            .route(
                "/order/{id}", get(purchases::order::get_order)
                .post(purchases::order::post_order),
            )
            // .route("/all", get(purchases::handlers::get_all))
            // .route(
            //     "/detail", get(purchases::creat::get_creat)
            //     .post(schedule::accreditation::post_password_change)
            // )
            .layer(Extension(Arc::new(purchases_tera))),
    );
    Router::new().without_v07_checks().merge(purchases_routes.with_state(state))
}
