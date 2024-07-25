pub mod common;

pub mod routes_assets;

pub mod routes_index;
pub mod routes_account;
pub mod routes_booking;
pub mod routes_provision;
pub mod routes_schedule;

pub mod util {
    pub mod date_config;
    // pub mod date_option;
}
pub mod auth {
    pub mod handlers;
    pub mod models;
    // pub mod repository;
    pub mod views;
}
pub mod profile {
    pub mod accreditation;
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
pub mod provision {
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
