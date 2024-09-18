use std::net::{Ipv4Addr, SocketAddr};

use tokio::net::TcpListener;

use sqlx::PgPool;
use axum::Router;

use axum_example::routes_assets;

use axum_example::chats::models::{RoomChat, UserChat};

use axum_example::routes_account;
use axum_example::routes_booking;
use axum_example::routes_index;
use axum_example::routes_provision;
use axum_example::routes_schedule;
use axum_example::routes_products;
use axum_example::routes_subscriptions;

use axum_example::routes_room_chats;
use axum_example::routes_user_chats;

use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;


#[tokio::main]
async fn main() {

    let cfg = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&cfg).await.unwrap();

    let assets_router = routes_assets::build_routes();

    let index_router = routes_index::build_routes(pool.clone());
    let account_router = routes_account::build_routes(pool.clone());
    let booking_router = routes_booking::build_routes(pool.clone());
    let schedule_router = routes_schedule::build_routes(pool.clone());
    let provision_router = routes_provision::build_routes(pool.clone());
    let product_router = routes_products::build_routes(pool.clone());
    let subscription_router = routes_subscriptions::build_routes(pool.clone());

    let chat_rm_state = Arc::new(RoomChat {
        rooms: Mutex::new(HashMap::new()),
        pool: pool.clone(),
    });

    let user_set = Mutex::new(HashSet::new());
    let (tx, _rx) = broadcast::channel(100);
    let chat_us_state = Arc::new(UserChat { user_set, tx, pool });

    let chat_rm_router = routes_room_chats::build_routes(chat_rm_state);
    let chat_us_router = routes_user_chats::build_routes(chat_us_state);

    let app = Router::new()
        .merge(assets_router)
        .merge(index_router)
        .merge(account_router)
        .merge(booking_router)
        .merge(provision_router)
        .merge(schedule_router)
        .merge(product_router)
        .merge(subscription_router)

        .merge(chat_rm_router)
        .merge(chat_us_router);

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8000));
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!(" listening on.. {:?}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
