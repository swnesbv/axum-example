use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use chrono::serde::ts_seconds_option;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CsvUser {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub img: Option<String>,
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExCsv {
    pub email: String,
    pub username: String,
    pub password: String,
    pub img: Option<String>,
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
}


#[derive(Debug, Clone, Deserialize)]
pub struct ExCsvUser {
    pub email: String,
    pub username: String,
    pub password: String,
    pub img: Option<String>,
}
