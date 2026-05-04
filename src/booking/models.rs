use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use chrono::serde::ts_seconds_option;

use crate::util::date_config::date_format;


#[derive(Deserialize, Serialize)]
pub struct FormNewBooking {
    pub title: String,
    pub description: Option<String>,
}
#[derive(Deserialize, Serialize)]
pub struct FormSearchPrd {
    pub start: Option<String>,
    pub end: Option<String>
}

#[derive(Serialize)]
pub struct ListBkg {
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
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>
}

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
    pub dates:   Option<Vec<NaiveDate>>,
    pub completed: bool,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Clone, Default, Serialize)]
pub struct CheckPrD {
    pub id: i32,
}

#[derive(Clone, Default, Serialize)]
pub struct CheckListPrD {
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
    pub updated_at: Option<DateTime<Utc>>
}
#[derive(Clone, Default, Serialize)]
pub struct CheckListPrH {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub st_hour: Option<NaiveDateTime>,
    pub en_hour: Option<NaiveDateTime>,
    pub s_hours: Option<Vec<NaiveDateTime>>,
    pub e_hours: Option<Vec<NaiveDateTime>>,
    pub hours:   Option<Vec<NaiveDateTime>>,
    pub completed: bool,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>
}
