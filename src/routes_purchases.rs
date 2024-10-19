use sqlx::postgres::PgPool;
use std::sync::Arc;

use axum::{routing::get, Extension, Router};

use tera::Tera;

use crate::{purchases};


pub fn build_routes(pool: PgPool) -> Router {
    let mut purchases_tera = Tera::default();
    purchases_tera
        .add_raw_templates(vec![
            ("base.html", include_str!("../templates/base.html")),
            ("navbar.html", include_str!("../templates/navbar.html")),
            (
                "all",
                include_str!("../templates/purchases/all.html"),
            ),
            ("order", include_str!("../templates/purchases/order.html")),
            // ("select", include_str!("../templates/purchases/select.html")),
            // ("categories", include_str!("../templates/purchases/categories.html")),
            // ("detail", include_str!("../templates/purchases/detail.html")),
            // ("delete", include_str!("../templates/schedule/delete.html")),
        ])
        .unwrap();

    let purchases_routes = Router::new().nest(
        "/purchases",
        Router::new()
            .route(
                "/order/:id", get(purchases::order::get_order)
                .post(purchases::order::post_order),
            )
            // .route("/all", get(purchases::handlers::get_all))
            // .route(
            //     "/detail", get(purchases::creat::get_creat)
            //     .post(schedule::accreditation::post_password_change)
            // )
            .layer(Extension(Arc::new(purchases_tera))),
    );
    Router::new().nest("/", purchases_routes.with_state(pool))
}
