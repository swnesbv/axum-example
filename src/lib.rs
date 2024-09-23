pub mod common;
pub mod routes_assets;
pub mod routes_index;
pub mod routes_account;
pub mod routes_booking;
pub mod routes_schedule;
pub mod routes_provision;
pub mod routes_products;
pub mod routes_subscriptions;
pub mod routes_room_chats;
pub mod routes_user_chats;
pub mod pgnation;

pub mod util {
    pub mod date_config;
    pub mod r_body;
}
pub mod auth {
    pub mod handlers;
    pub mod models;
    pub mod repository;
    pub mod views;
}
pub mod profile {
    pub mod accreditation;
    pub mod handlers;
    pub mod models;
    // pub mod repository;
    pub mod views;
}
pub mod comments {
    pub mod creat;
    // pub mod handlers;
    pub mod models;
    // pub mod repository;
    pub mod views;
}
pub mod products {
    pub mod creat;
    pub mod handlers;
    pub mod models;
    // pub mod repository;
    pub mod views;
}
pub mod import_export {
    pub mod handlers;
    pub mod models;
    pub mod repository;
    pub mod views;
}
pub mod photo {
    pub mod handlers;
    pub mod models;
    pub mod repository;
    pub mod views;
}
pub mod booking {
    pub mod creat;
    pub mod handlers;
    pub mod models;
    pub mod views;
}
pub mod schedule {
    pub mod creat;
    pub mod handlers;
    pub mod models;
    pub mod views;
}
pub mod provision {
    pub mod creat;
    pub mod handlers;
    pub mod models;
    pub mod views;
}
pub mod subscriptions {
    pub mod creat;
    pub mod handlers;
    pub mod models;
    pub mod repository;
    pub mod views;
}
pub mod chats {
    pub mod handler_rm;
    pub mod handler_us;
    pub mod handlers;
    pub mod models;
    pub mod repository;
    pub mod views;
}
