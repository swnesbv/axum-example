use axum::Router;

use sqlx::PgPool;

use tracing::info;

use std::net::{Ipv4Addr, SocketAddr};

use tokio::net::TcpListener;

use axum_example::routes_assets;

use axum_example::routes_index;
use axum_example::routes_account;
use axum_example::routes_booking;
use axum_example::routes_provision;

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();

    let cfg = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&cfg).await.unwrap();

    let assets_router = routes_assets::build_routes();

    let index_router = routes_index::build_routes(pool.clone());
    let account_router = routes_account::build_routes(pool.clone());
    let booking_router = routes_booking::build_routes(pool.clone());
    let provision_router = routes_provision::build_routes(pool.clone());

    let app = Router::new()
        .merge(assets_router)
        .merge(index_router)
        .merge(account_router)
        .merge(booking_router)
        .merge(provision_router);

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8000));
    let listener = TcpListener::bind(&addr).await.unwrap();
    info!("Listening on {}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
