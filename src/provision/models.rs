use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::util::date_config::date_format;
use chrono::serde::ts_seconds_option;

#[derive(Serialize)]
pub struct AllPrD {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub st_date: Option<NaiveDate>,
    pub en_date: Option<NaiveDate>,
    pub s_dates: Option<Vec<NaiveDate>>,
    pub e_dates: Option<Vec<NaiveDate>>,
    pub dates: Option<Vec<NaiveDate>>,
    pub completed: bool,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
}
#[derive(Serialize)]
pub struct AllPrH {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub st_hour: Option<NaiveDateTime>,
    pub en_hour: Option<NaiveDateTime>,
    pub s_hours: Option<Vec<NaiveDateTime>>,
    pub e_hours: Option<Vec<NaiveDateTime>>,
    pub hours: Option<Vec<NaiveDateTime>>,
    pub completed: bool,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize)]
pub struct FormPrD {
    pub title: String,
    pub description: Option<String>,
    pub st_date: Option<String>,
    pub en_date: Option<String>,
}
#[derive(Deserialize, Serialize)]
pub struct FormPrH {
    pub title: String,
    pub description: Option<String>,
    pub st_hour: Option<String>,
    pub en_hour: Option<String>,
}
#[derive(Deserialize, Serialize)]
pub struct FormPrdBkg {
    pub title: String,
    pub description: Option<String>,
    pub s_dates: NaiveDate,
    pub e_dates: NaiveDate,
}

#[derive(Serialize)]
pub struct UpPrD {
    pub title: String,
    pub description: Option<String>,
    pub st_date: Option<NaiveDate>,
    pub en_date: Option<NaiveDate>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
pub struct BkgPrD {
    pub user_id: i32,
    pub provision_d_id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub st_date: Option<NaiveDate>,
    pub en_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct UpPrdBkg {
    pub s_dates: Vec<NaiveDate>,
    pub e_dates: Vec<NaiveDate>,
    pub dates: Vec<NaiveDate>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
}
