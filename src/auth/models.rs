use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::util::date_config::date_format;
use chrono::serde::ts_seconds_option;

#[derive(Debug, Clone, Serialize)]
pub struct ListUser {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub img: Option<String>,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormLogin {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub exp: usize,
}
