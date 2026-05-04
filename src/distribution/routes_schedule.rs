use sqlx::postgres::PgPool;
use std::sync::Arc;
use axum::{routing::get, Extension, Router};
use tera::Tera;

use crate::schedule;

pub fn build_routes(pool: PgPool) -> Router {
    let mut schedule_tera = Tera::default();
    schedule_tera
        .add_raw_templates(vec![
            ("base.html", include_str!("../../tps/base.html")),
            ("navbar.html", include_str!("../../tps/navbar.html")),
            ("rq_user.html", include_str!("../../tps/rq_user.html")),
            (
                "all_sch",
                include_str!("../../tps/schedule/all_sch.html"),
            ),
            (
                "all_recording",
                include_str!("../../tps/schedule/all_recording.html"),
            ),
            ("creat", include_str!("../../tps/schedule/creat.html")),
            ("select", include_str!("../../tps/schedule/select.html")),
            ("places", include_str!("../../tps/schedule/places.html")),
            // ("detail", include_str!("../../tps/schedule/detail.html")),
            // ("delete", include_str!("../../tps/schedule/delete.html")),
        ])
        .unwrap();

    let schedule_routes = Router::new().nest(
        "/schedule",
        Router::new()
            .route(
                "/creat",
                get(schedule::creat::get_creat).post(schedule::creat::post_creat),
            )
            .route("/all-sch", get(schedule::handlers::get_all_sch))
            .route("/all-recording", get(schedule::handlers::get_all_recording))
            .route(
                "/select",
                get(schedule::handlers::get_select).post(schedule::handlers::post_select),
            )
            .route(
                "/places",
                get(schedule::handlers::get_places).post(schedule::handlers::post_places),
            )
            // .route(
            //     "/detail", get(schedule::accreditation::get_password_change).post(schedule::accreditation::post_password_change)
            // )
            .layer(Extension(Arc::new(schedule_tera))),
    );
    Router::new().merge(schedule_routes.with_state(pool))
}
