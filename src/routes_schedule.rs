use sqlx::postgres::PgPool;
use std::sync::Arc;

use axum::{routing::get, Extension, Router};

use tera::Tera;

use crate::{schedule};


pub fn build_routes(pool: PgPool) -> Router {

    let mut schedule_tera = Tera::default();
    schedule_tera
        .add_raw_templates(vec![
            ("base.html", include_str!("../templates/base.html")),
            ("navbar.html", include_str!("../templates/navbar.html")),
            ("all", include_str!("../templates/schedule/all.html")),
            (
                "creat",
                include_str!("../templates/schedule/creat.html"),
            ),
            (
                "select",
                include_str!("../templates/schedule/select.html"),
            ),
            // ("detail", include_str!("../templates/schedule/detail.html")),
            // ("delete", include_str!("../templates/schedule/delete.html")),

        ])
        .unwrap();

    let schedule_routes = Router::new().nest(
        "/schedule",
        Router::new()
            .route(
                "/creat",
                get(schedule::creat::get_creat).post(schedule::creat::post_creat),
            )
            .route(
                "/select",
                get(schedule::handlers::get_select).post(schedule::handlers::post_select),
            )
            .route(
                "/all",
                get(schedule::handlers::get_all)
            )
            // .route(
            //     "/detail", get(schedule::accreditation::get_password_change).post(schedule::accreditation::post_password_change)
            // )
            .layer(Extension(Arc::new(schedule_tera))),
    );
    Router::new().nest("/", schedule_routes.with_state(pool))
}
