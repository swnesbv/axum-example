use std::sync::Arc;
use axum::{routing::get, Extension, Router};
use tera::Tera;

use crate::{
    photo::creat,
    photo::handlers,
    auth::models::{AuthRedis},
};

pub fn rt(state: Arc<AuthRedis>) -> Router {
    let mut photo_tera = Tera::default();
    photo_tera
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
                "completed.html",
                include_str!("../../tps/element/completed.html")
            ),
            (
                "creat",
                include_str!("../../tps/photo/creat.html")
            ),
            (
                "photo",
                include_str!("../../tps/photo/photo.html")
            ),
            (
                "slider_photo",
                include_str!("../../tps/photo/slider_photo.html")
            ),
            (
                "collections",
                include_str!("../../tps/photo/collections.html")
            ),
            (
                "update_slider",
                include_str!("../../tps/photo/update_slider.html")
            ),
        ])
        .unwrap();

    let photo_routes = Router::new().without_v07_checks()
        .route(
            "/photo",
            get(creat::get_photo)
            .post(creat::post_photo_user),
        )
        .route(
            "/creat-slider",
            get(creat::get_creat_slider)
            .post(creat::post_creat_slider),
        )
        .route(
            "/photo-zip",
            get(creat::get_photo)
            .post(creat::post_collections_zip),
        )
        .route(
            "/collections",
            get(handlers::get_collections)
        )
        .without_v07_checks()
        .route(
            "/photo-zip/{id}",
            get(handlers::get_slider_photo)
        )
        .route(
            "/update-slider/{id}/product/{to_product}",
            get(handlers::get_update_slider)
            .post(handlers::post_update_slider),
        )
        .layer(Extension(Arc::new(photo_tera)));
    Router::new().merge(photo_routes.with_state(state))
}
