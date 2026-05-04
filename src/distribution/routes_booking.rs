use std::sync::Arc;
use axum::{routing::get, Extension, Router};
use tera::Tera;

use crate::{
    booking,
    auth::models::{AuthRedis},
};

pub fn rt(state: Arc<AuthRedis>) -> Router {
    let mut booking_tera = Tera::default();
    booking_tera
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
                "created_updated.html",
                include_str!("../../tps/element/created_updated.html")
            ),
            (
                "completed.html",
                include_str!("../../tps/element/completed.html")
            ),
            (
                "all_booking",
                include_str!("../../tps/booking/all_booking.html"),
            ),
            (
                "matching_days",
                include_str!("../../tps/booking/matching_days.html")
            ),
            (
                "matching_hours",
                include_str!("../../tps/booking/matching_hours.html")
            ),
            (
                "search_days",
                include_str!("../../tps/booking/search_days.html"),
            ),
            (
                "search_hours",
                include_str!("../../tps/booking/search_hours.html"),
            ),
            // ("delete", include_str!("../../tps/booking/delete.html")),
            // ("detail", include_str!("../../tps/booking/detail.html")),
            // ("update", include_str!("../../tps/booking/update.html")),
        ])
        .unwrap();

    let booking_routes = Router::new().nest(
        "/booking",
        Router::new()
            .route("/all-booking", get(booking::handlers::bkg_all))
            .route(
                "/matching-days",
                get(booking::handlers::get_matching_days)
                .post(booking::creat::post_matching_days)
            )
            .route(
                "/search-days",
                get(booking::handlers::get_search_days)
                .post(booking::creat::post_search_days),
            )
            .route(
                "/matching-hours",
                get(booking::handlers::get_matching_hours)
                .post(booking::creat::post_matching_hours)
            )
            .route(
                "/search-hours",
                get(booking::handlers::get_search_hours)
                .post(booking::creat::post_search_hours),
            )
            /*.route(
                "/creat-period-hours", get(profile::accreditation::get_signup).post(profile::accreditation::post_signup)
            )
            .route(
                "/search-period-hours", get(profile::accreditation::get_password_change).post(profile::accreditation::post_password_change)
            )*/
            .layer(Extension(Arc::new(booking_tera))),
    );
    Router::new().merge(booking_routes.with_state(state))
}
