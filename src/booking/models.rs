use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::util::date_config::date_format;
use chrono::serde::ts_seconds_option;

#[derive(Debug, Deserialize, Serialize)]
pub struct NewBooking {
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FormNewBooking {
    pub title: String,
    pub description: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct FormSE {
    pub start: NaiveDate,
    pub end: NaiveDate,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub start: NaiveDate,
    pub end: NaiveDate,
}


#[derive(Debug, Serialize)]
pub struct LtBkg {
    pub id: i32,
    pub user_id: i32,
    pub provision_d_id: Option<i32>,
    pub provision_h_id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub st_date: Option<NaiveDate>,
    pub en_date: Option<NaiveDate>,
    pub st_hour: Option<NaiveDateTime>,
    pub en_hour: Option<NaiveDateTime>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}



#[derive(Debug, Serialize)]
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


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SllPrD {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub st_date: Option<NaiveDate>,
    pub en_date: Option<NaiveDate>,
    pub s_dates: Option<Vec<NaiveDate>>,
    pub e_dates: Option<Vec<NaiveDate>>,
    pub dates:   Option<Vec<NaiveDate>>,
    pub completed: bool,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
}