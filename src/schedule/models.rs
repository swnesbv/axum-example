use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use chrono::serde::ts_seconds_option;

use crate::util::date_config::date_format;
use crate::util::r_body::deserialize_list;


#[derive(Serialize)]
pub struct Title {
    pub title: String,
}
#[derive(Serialize)]
pub struct Schedule {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub st_hour: Option<NaiveDateTime>,
    pub en_hour: Option<NaiveDateTime>,
    pub hours: Option<Vec<NaiveDateTime>>,
    pub occupied: Option<Vec<NaiveDateTime>>,
    pub places: Option<Vec<i32>>,
    pub non_places: Option<Vec<i32>>,
    pub completed: bool,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
pub struct Places {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub hours: Option<Vec<NaiveDateTime>>,
    pub places: Option<Vec<i32>>,
    pub non_places: Option<Vec<i32>>,
    pub completed: bool,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
}


#[derive(Serialize)]
pub struct Tickets {
    pub to_schedule: i32,
    pub title: String,
    pub record_h: NaiveDateTime,
    pub places: Vec<i32>,
}

#[derive(Serialize)]
pub struct Recording {
    pub id: i32,
    pub user_id: i32,
    pub to_schedule: i32,
    pub record_d: Option<NaiveDate>,
    pub record_h: Option<NaiveDateTime>,
    pub places: Option<Vec<i32>>,
    pub tickets: Option<sqlx::types::JsonValue>,
    pub completed: bool,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FormSch {
    pub title: String,
    pub description: Option<String>,
    pub st_hour: Option<String>,
    pub en_hour: Option<String>,
    pub places: Option<i32>,
    #[serde(flatten, deserialize_with = "deserialize_list")]
    pub list: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize)]
pub struct FormSelect {
    pub to_schedule: i32,
    pub record_d: NaiveDate,
    pub record_h: NaiveDateTime,
}

#[derive(Deserialize, Serialize)]
pub struct FormPlaces {
    pub to_schedule: i32,
    pub record_h: NaiveDateTime,
    pub places: Vec<i32>,
    pub on_off: Vec<String>,
}


/*#[derive(Debug, Deserialize, Serialize)]
pub struct FormSch {
    pub title:       String,
    pub description: Option<String>,
    pub st_hour:     Option<String>,
    pub en_hour:     Option<String>,
    pub vec_list:    Option<String>,
}*/
