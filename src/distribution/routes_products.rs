use std::sync::Arc;
use axum::{routing::get, Extension, Router};
use tera::Tera;

use crate::{
    products,
    auth::models::{AuthRedis}
};


pub fn rt(state: Arc<AuthRedis>) -> Router {
    let mut products_tera = Tera::default();
    products_tera
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
                include_str!("../../tps/products/all.html"),
            ),
            ("creat", include_str!("../../tps/products/creat.html")),
            ("select", include_str!("../../tps/products/select.html")),
            ("categories", include_str!("../../tps/products/categories.html")),
            ("cts", include_str!("../../tps/products/cts.html")),
            ("detail", include_str!("../../tps/products/detail.html")),
            // ("delete", include_str!("../../tps/products/delete.html")),
        ])
        .unwrap();

    let products_routes = Router::new().without_v07_checks().nest(
        "/products",
        Router::new()
            .route(
                "/creat",
                get(products::creat::get_creat).post(products::creat::post_creat),
            )
            .route("/all", get(products::handlers::get_all))
            .route(
                "/select",
                get(products::handlers::get_select).post(products::handlers::post_select),
            )
            .without_v07_checks()
            .route("/categories/{i}", get(products::handlers::get_categories))
            .route("/cts/{i}", get(products::handlers::get_cts))
            .route(
                "/detail/{id}", get(products::handlers::get_detail)
            )
            .layer(Extension(Arc::new(products_tera))),
    );
    Router::new().without_v07_checks().merge(products_routes.with_state(state))
}
