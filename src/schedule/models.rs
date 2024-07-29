use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::util::date_config::date_format;
use chrono::serde::ts_seconds_option;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Schedule {
    pub id:          i32,
    pub user_id:     i32,
    pub title:       String,
    pub description: Option<String>,
    pub st_hour:     Option<NaiveDateTime>,
    pub en_hour:     Option<NaiveDateTime>,
    pub hours:       Option<Vec<NaiveDateTime>>,
    pub occupied:    Option<Vec<NaiveDateTime>>,
    pub completed:   bool,
    #[serde(with = "date_format")]
    pub created_at:  DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at:  Option<DateTime<Utc>>,
}


#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct FormSch {
    pub title:       String,
    pub description: Option<String>,
    pub st_hour:     Option<String>,
    pub en_hour:     Option<String>,
    pub list:        Option<Vec<String>>,
}


/*#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct FormSch {
    pub title:       String,
    pub description: Option<String>,
    pub st_hour:     Option<String>,
    pub en_hour:     Option<String>,
    pub vec_list:    Option<String>,
}*/


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormSelect {
    pub to_schedule: i32,
    pub record_d:    NaiveDate,
    pub record_h:    NaiveDateTime,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
}
