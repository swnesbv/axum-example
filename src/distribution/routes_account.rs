use sqlx::postgres::PgPool;
use std::sync::Arc;

use axum::{routing::get, Extension, Router};

use tera::Tera;

use crate::{
    auth, import_export, photo, profile, comments, subscriptions
};

pub fn build_routes(pool: PgPool) -> Router {
    let mut user_tera = Tera::default();
    user_tera
        .add_raw_templates(vec![
            ("base.html", include_str!("../../templates/base.html")),
            ("navbar.html", include_str!("../../templates/navbar.html")),
            ("users", include_str!("../../templates/users.html")),
            ("user", include_str!("../../templates/user.html")),
            ("signup", include_str!("../../templates/signup.html")),
            ("login", include_str!("../../templates/login.html")),
            ("update", include_str!("../../templates/update.html")),
            (
                "password_change",
                include_str!("../../templates/password_change.html"),
            ),
            ("export_csv", include_str!("../../templates/export_csv.html")),
            ("photo", include_str!("../../templates/photo.html")),
        ])
        .unwrap();

    let auth_routes = Router::new().nest(
        "/account",
        Router::new()
            .route(
                "/users",
                get(profile::handlers::users)
                .post(subscriptions::creat::post_ssc_user)
            )
            .route("/user/:name", get(profile::handlers::user).post(comments::creat::post_creat))
            .route(
                "/login",
                get(auth::handlers::get_login).post(auth::handlers::post_login),
            )
            .route(
                "/signup",
                get(profile::accreditation::get_signup).post(profile::accreditation::post_signup),
            )
            .route(
                "/update",
                get(profile::accreditation::get_update)
                    .post(profile::accreditation::post_update_user),
            )
            .route(
                "/password-change",
                get(profile::accreditation::get_password_change)
                    .post(profile::accreditation::post_password_change),
            )
            .route("/import", get(import_export::handlers::import_users))
            .route(
                "/export",
                get(import_export::handlers::get_export_users)
                    .post(import_export::handlers::export_users),
            )
            .route(
                "/photo",
                get(photo::handlers::get_photo_users).post(photo::handlers::photo_users),
            )
            .layer(Extension(Arc::new(user_tera))),
    );
    Router::new().nest("/", auth_routes.with_state(pool))
}
