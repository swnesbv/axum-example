use axum::{
    http::header::{HeaderMap},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use chrono::serde::ts_seconds_option;

use crate::{
    common::{PgPool, RedisPool},
    auth::check::{in_check},
    util::date_config::date_format
};

#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct KeyEmail {
    pub key:   String,
    pub email: String,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct AuToken {
    pub id:       i32,
    pub email:    String,
    pub username: String,
    pub status:   Vec<String>
}
#[derive(Clone, Debug)]
pub struct AuthRedis {
    pub pool: PgPool,
    pub conn: RedisPool
}
impl AuthRedis {
    pub async fn ctx(
        &self, headers: HeaderMap
    ) -> Result<Option<AuToken>, Option<String>> {
        let conn = &self.conn;
        match in_check(conn.clone(), headers).await {
            Ok(expr) => Ok(expr),
            Err(_) => Ok(None),
        }
    }
}


#[derive(Debug)]
pub struct AdminStatus {
    pub status: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct VeriUser {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub status: Vec<String>
}

#[derive(Deserialize, Serialize)]
pub struct FormLogin {
    pub email: String,
    pub password: String
}

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub status: Vec<String>,
    pub exp: usize
}


#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AllUser {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub img: Option<String>,
    pub status: Vec<String>,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>
}