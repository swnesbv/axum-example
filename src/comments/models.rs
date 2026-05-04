use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use chrono::serde::ts_seconds_option;
use crate::util::date_config::date_format;


#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FormComment {
    pub to_id:   Option<i32>,
    pub comment: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd, Ord, Eq)]
pub struct Cmt {
    pub user_id:   i32,
    pub tab_id:    i32,
    pub email:     String,
    pub name:      String,
    pub msg:       String,
    pub completed: bool,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Comment {
    pub comments: Option<Vec<serde_json::Value>>
}
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct VecCmt {
    pub comments: Vec<Vec<Cmt>>
}