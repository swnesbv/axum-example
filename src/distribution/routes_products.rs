use sqlx::postgres::PgPool;
use std::sync::Arc;

use axum::{routing::get, Extension, Router};

use tera::Tera;

use crate::products;


pub fn build_routes(pool: PgPool) -> Router {
    let mut products_tera = Tera::default();
    products_tera
        .add_raw_templates(vec![
            ("base.html", include_str!("../../templates/base.html")),
            ("navbar.html", include_str!("../../templates/navbar.html")),
            (
                "all",
                include_str!("../../templates/products/all.html"),
            ),
            ("creat", include_str!("../../templates/products/creat.html")),
            ("select", include_str!("../../templates/products/select.html")),
            ("categories", include_str!("../../templates/products/categories.html")),
            ("detail", include_str!("../../templates/products/detail.html")),
            // ("delete", include_str!("../../templates/products/delete.html")),
        ])
        .unwrap();

    let products_routes = Router::new().nest(
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
            .route("/categories/:i", get(products::handlers::get_categories))
            .route(
                "/detail/:id", get(products::handlers::get_detail)
            )
            .layer(Extension(Arc::new(products_tera))),
    );
    Router::new().nest("/", products_routes.with_state(pool))
}
