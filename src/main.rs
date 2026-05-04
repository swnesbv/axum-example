use std::net::{Ipv4Addr, SocketAddr};
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;
use tokio::net::TcpListener;
use tokio_postgres::NoTls;
use bb8_postgres::PostgresConnectionManager;
use axum::Router;

use axum_example::auth::models::{AuthRedis};
use axum_example::chats::models::{RoomChat, UserChat};

use axum_example::distribution::routes_assets;

use axum_example::distribution::routes_photo;

use axum_example::distribution::routes_index;
use axum_example::distribution::routes_account;
use axum_example::distribution::routes_booking;
use axum_example::distribution::routes_provision;
use axum_example::distribution::routes_room_chats;
use axum_example::distribution::routes_user_chats;
use axum_example::distribution::routes_subscriptions;



#[tokio::main]
async fn main() {

    //..Postgres
    let cfg = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = PostgresConnectionManager::new_from_stringlike(cfg, NoTls).unwrap();
    let pool = bb8::Pool::builder().build(manager).await.unwrap();
    //..Redis
    let client = redis::Client::open("redis://localhost").unwrap();
    let conn = bb8::Pool::builder().build(client).await.unwrap();
    // ..

    let a = AuthRedis {
        pool: pool.clone(),
        conn: conn.clone()
    };
    let assets_router = routes_assets::build_rt();

    let photo_router = routes_photo::rt(Arc::new(a.clone()));
    let index_router = routes_index::rt(Arc::new(a.clone()));
    let booking_router = routes_booking::rt(Arc::new(a.clone()));
    let provision_router = routes_provision::rt(Arc::new(a.clone()));
    let account_router = routes_account::rt(Arc::new(a.clone())).await;
    let subscription_router = routes_subscriptions::rt(Arc::new(a.clone()));


    let b = RoomChat {
        rooms: Mutex::new(HashMap::new()),
        pool: pool.clone(),
        conn: conn.clone()
    };
    let chat_rm_router = routes_room_chats::rt(Arc::new(b));

    let user_set = Mutex::new(HashSet::new());
    let (tx, _rx) = broadcast::channel(100);

    let c = UserChat {
        user_set,
        tx,
        pool: pool.clone(),
        conn: conn.clone()
    };
    let chat_us_state = Arc::new(c);
    let chat_us_router = routes_user_chats::rt(chat_us_state);

    let app = Router::new()
        .merge(assets_router)
        .merge(index_router)
        .without_v07_checks()
        .merge(account_router)
        .merge(subscription_router)
        .without_v07_checks()
        .merge(chat_rm_router)
        .merge(chat_us_router)
        .without_v07_checks()
        .merge(provision_router)
        .merge(booking_router)
        .merge(photo_router);

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8000));
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!(" listening on.. {:?}", addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
