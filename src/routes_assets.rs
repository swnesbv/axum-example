use axum::Router;

use std::{path::PathBuf};
use tower_http::services::ServeDir;


pub fn build_routes() -> Router {

    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static");
    Router::new()
        .fallback_service(
            ServeDir::new(assets_dir).append_index_html_on_directories(true)
        )
}
