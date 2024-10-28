use sqlx::postgres::PgPool;
use std::sync::Arc;

use axum::{routing::get, Extension, Router};

use tera::Tera;

use crate::provision;

pub fn build_routes(pool: PgPool) -> Router {
    let mut provision_tera = Tera::default();
    provision_tera
        .add_raw_templates(vec![
            ("base.html", include_str!("../../templates/base.html")),
            ("navbar.html", include_str!("../../templates/navbar.html")),
            (
                "all_days",
                include_str!("../../templates/provision/all_days.html"),
            ),
            (
                "all_hours",
                include_str!("../../templates/provision/all_hours.html"),
            ),
            (
                "creat_days",
                include_str!("../../templates/provision/creat_days.html"),
            ),
            // ("delete", include_str!("../../templates/provision/period.html")),
            (
                "detail_days",
                include_str!("../../templates/provision/detail_days.html"),
            ),
            (
                "update_dates",
                include_str!("../../templates/provision/update_dates.html"),
            ),
            (
                "creat_hours",
                include_str!("../../templates/provision/creat_hours.html"),
            ),
            // ("detail_hours", include_str!("../../templates/provision/export_csv.html")),
        ])
        .unwrap();

    let provision_routes = Router::new().nest(
        "/provision",
        Router::new()
            .route(
                "/creat-days",
                get(provision::creat::get_creat_days).post(provision::creat::post_creat_days),
            )
            .route(
                "/update-days/:prv_id",
                get(provision::creat::get_update_days).post(provision::creat::post_update_days),
            )
            .route("/all-days", get(provision::handlers::get_all_days))
            .route(
                "/detail-days/:prv_id",
                get(provision::handlers::get_detail_days)
                    .post(provision::handlers::post_detail_days),
            )
            .route(
                "/creat-hours",
                get(provision::creat::get_creat_hours).post(provision::creat::post_creat_hours),
            )
            .route("/all-hours", get(provision::handlers::get_all_hours))
            // .route(
            //     "/detail-hours", get(profile::accreditation::get_password_change).post(profile::accreditation::post_password_change)
            // )
            .layer(Extension(Arc::new(provision_tera))),
    );
    Router::new().nest("/", provision_routes.with_state(pool))
}
