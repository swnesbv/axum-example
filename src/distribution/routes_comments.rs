use std::sync::Arc;

use axum::{routing::{get}, Extension, Router};

use tera::Tera;

use crate::{
    comments,
    auth::models::{AuthRedis},
};

pub fn rt(state: Arc<AuthRedis>) -> Router {
    let mut comments_tera = Tera::default();
    comments_tera
        .add_raw_templates(vec![
            ("base.html", include_str!("../../tps/base.html")),
            ("navbar.html", include_str!("../../tps/element/navbar.html")),
            (
                "rq_user.html",
                include_str!("../../tps/element/rq_user.html")
            ),
            (
                "update_cmt", include_str!("../../tps/comments/update_cmt.html")
            ),
            (
                "cmt_del", include_str!("../../tps/comments/cmt_del.html")
            ),
        ])
        .unwrap();

    let comments_routes = Router::new().without_v07_checks()
            .route(
                "/update-cmt/user/{name}/cmt/{cid}",
                get(comments::handlers::get_update_cmt)
                .post(comments::handlers::post_update_cmt)
            )
            .route(
                "/del-cmt/user/{name}/cmt/{cid}",
                get(comments::handlers::get_cmt_del)
                .post(comments::handlers::post_cmt_del)
            )
        .layer(Extension(Arc::new(comments_tera)));
    Router::new().merge(comments_routes.with_state(state))
}
