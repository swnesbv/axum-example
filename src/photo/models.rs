use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds_option;

use crate::util::date_config::date_format;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormSlider {
    pub to_product:  i32,
    pub title:       String,
    pub description: String,
    pub img:         Vec<String>,
    pub on_off:      Vec<String>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Slider {
    pub id:          i32,
    pub user_id:     i32,
    pub to_product:  Option<i32>,
    pub title:       Option<serde_json::Value>,
    pub description: Option<serde_json::Value>,
    pub img:         Option<serde_json::Value>,
    pub completed:   bool,
    #[serde(with = "date_format")]
    pub created_at:  DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at:  Option<DateTime<Utc>>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Collections {
    pub id:          i32,
    pub user_id:     i32,
    pub to_product:  i32,
    pub img:         Vec<String>,
    pub completed:   bool,
    #[serde(with = "date_format")]
    pub created_at:  DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at:  Option<DateTime<Utc>>
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Img {
    pub img: Option<Vec<serde_json::Value>>
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd, Ord, Eq)]
pub struct ZipImg {
    pub path: String,
}
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct VecImg {
    pub img: Vec<Vec<ZipImg>>
}


#[derive(Deserialize, Serialize)]
pub struct Msg {
    pub msg: String,
    pub alert: String
}