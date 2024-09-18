use sqlx::postgres::PgPool;
use std::sync::Arc;

use axum::{routing::get, Extension, Router};

use tera::Tera;

use crate::booking;

pub fn build_routes(pool: PgPool) -> Router {
    let mut booking_tera = Tera::default();
    booking_tera
        .add_raw_templates(vec![
            ("base.html", include_str!("../templates/base.html")),
            ("navbar.html", include_str!("../templates/navbar.html")),
            (
                "all_booking",
                include_str!("../templates/booking/all_booking.html"),
            ),
            ("period", include_str!("../templates/booking/period.html")),
            (
                "search_days",
                include_str!("../templates/booking/search_days.html"),
            ),
            // ("delete", include_str!("../templates/booking/delete.html")),
            // ("detail", include_str!("../templates/booking/detail.html")),

            // ("period_hours", include_str!("../templates/booking/period_hours.html")),
            // ("update", include_str!("../templates/booking/update.html")),
        ])
        .unwrap();

    let booking_routes = Router::new().nest(
        "/booking",
        Router::new()
            .route("/all-booking", get(booking::handlers::bkg_all))
            .route(
                "/creat-period-days",
                get(booking::creat::get_period).post(booking::creat::post_period),
            )
            .route("/search-period-days", get(booking::handlers::search_days))
            /*.route(
                "/creat-period-hours", get(profile::accreditation::get_signup).post(profile::accreditation::post_signup)
            )
            .route(
                "/search-period-hours", get(profile::accreditation::get_password_change).post(profile::accreditation::post_password_change)
            )*/
            .layer(Extension(Arc::new(booking_tera))),
    );
    Router::new().nest("/", booking_routes.with_state(pool))
}
