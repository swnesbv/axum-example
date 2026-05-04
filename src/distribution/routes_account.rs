use std::sync::Arc;
use axum::{routing::get, Extension, Router};
use tera::Tera;

use crate::{
    profile,
    comments,
    subscriptions,
    import_export,
    auth::handlers,
    auth::models::{AuthRedis}
};

pub async fn rt(state: Arc<AuthRedis>) -> Router {
    let mut user_tera = Tera::default();
    user_tera
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
                "password_change",
                include_str!("../../tps/profile/password_change.html"),
            ),
            ("update", include_str!("../../tps/profile/update.html")),
            ("users", include_str!("../../tps/profile/users.html")),
            ("user", include_str!("../../tps/profile/user.html")),
            ("signup", include_str!("../../tps/profile/signup.html")),
            ("login", include_str!("../../tps/login.html")),
            (
                "export_csv",
                include_str!("../../tps/element/export_csv.html")
            )
        ])
        .unwrap();

    let auth_routes = Router::new().without_v07_checks().nest(
        "/account",
        Router::new()
            .route(
                "/users",
                get(profile::handlers::users)
                .post(subscriptions::creat::post_ssc_user)
            )
            .without_v07_checks()
            .route("/user/{name}",
                get(profile::handlers::user)
                .post(comments::creat::post_creat_cmt_user)
            )
            .route(
                "/signup",
                get(profile::accreditation::get_signup)
                .post(profile::accreditation::post_signup)
            )
            .route(
                "/login",
                get(handlers::get_login)
                .post(handlers::post_login)
            )
            .route(
                "/update",
                get(profile::accreditation::get_update_user)
                .post(profile::accreditation::post_update_user)
            )
            .route(
                "/password-change",
                get(profile::accreditation::get_password_change)
                .post(profile::accreditation::post_password_change)
            )
            .without_v07_checks()
            .route(
                "/delete-user/{uaer_id}",
                get(profile::accreditation::get_del_user)
            )
            .route(
                "/import-users",
                get(import_export::handlers::import_users)
            )
            .route(
                "/export-users",
                get(import_export::handlers::get_export_users)
                .post(import_export::handlers::post_export_users)
            )
            .layer(Extension(Arc::new(user_tera.clone())))
    );
    Router::new()
        .without_v07_checks()
        .merge(auth_routes.with_state(state))

}
