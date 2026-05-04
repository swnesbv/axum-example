use std::sync::Arc;
use axum::{routing::get, Extension, Router};
use tera::Tera;

use crate::{
    provision,
    auth::models::{AuthRedis},
};

pub fn rt(state: Arc<AuthRedis>) -> Router {
    let mut provision_tera = Tera::default();
    provision_tera
        .add_raw_templates(vec![
            ("base.html", include_str!("../../tps/base.html")),
            ("navbar.html", include_str!("../../tps/navbar.html")),
            (
                "rq_user.html",
                include_str!("../../tps/element/rq_user.html")
            ),
            (
                "st_en_date.html",
                include_str!("../../tps/element/st_en_date.html")
            ),
            (
                "st_en_hour.html",
                include_str!("../../tps/element/st_en_hour.html")
            ),
            (
                "comments.html",
                include_str!("../../tps/element/comments.html")
            ),
            (
                "created_updated.html",
                include_str!("../../tps/element/created_updated.html")
            ),
            (
                "completed.html",
                include_str!("../../tps/element/completed.html")
            ),
            ("all_days",
                include_str!("../../tps/provision/all_days.html"),
            ),
            (
                "all_hours",
                include_str!("../../tps/provision/all_hours.html")
            ),
            (
                "creat_days",
                include_str!("../../tps/provision/creat_days.html")
            ),
            (
                "detail_days",
                include_str!("../../tps/provision/detail_days.html")
            ),
            (
                "detail_hours",
                include_str!("../../tps/provision/detail_hours.html")
            ),
            (
                "update_dates",
                include_str!("../../tps/provision/update_dates.html")
            ),
            (
                "creat_hours",
                include_str!("../../tps/provision/creat_hours.html")
            ),
            (
                "detail_hours",
                include_str!("../../tps/provision/detail_hours.html")
            ),
            (
                "update_hours",
                include_str!("../../tps/provision/update_hours.html")
            ),
        ])
        .unwrap();

    let provision_routes = Router::new().without_v07_checks().nest(
        "/provision",
        Router::new()
            .route("/all-days", get(provision::handlers::get_all_days))
            .route("/all-hours", get(provision::handlers::get_all_hours))
            .route(
                "/creat-days",
                get(provision::creat::get_creat_days)
                .post(provision::creat::post_creat_days)
            )
            .without_v07_checks()
            .route(
                "/update-days/{prv_id}",
                get(provision::creat::get_update_days)
                .post(provision::creat::post_update_days)
            )
            .without_v07_checks()
            .route(
                "/detail-days/{prv_id}",
                get(provision::handlers::get_detail_days)
                .post(provision::handlers::post_detail_days)
            )
            .without_v07_checks()
            .route(
                "/delete-days/{prv_id}",
                get(provision::creat::get_del_days)
            )
            .route(
                "/creat-hours",
                get(provision::creat::get_creat_hours)
                .post(provision::creat::post_creat_hours)
            )
            .without_v07_checks()
            .route(
                "/update-hours/{prv_id}",
                get(provision::creat::get_update_hours)
                .post(provision::creat::post_update_hours)
            )
            .without_v07_checks()
            .route(
                "/detail-hours/{prv_id}",
                get(provision::handlers::get_detail_hours)
                .post(provision::handlers::post_detail_hours)
            )
            .without_v07_checks()
            .route(
                "/delete-hours/{prv_id}",
                get(provision::creat::get_del_days)
            )
            .layer(Extension(Arc::new(provision_tera)))
    );
    Router::new().without_v07_checks()
    .merge(provision_routes.with_state(state))
}
