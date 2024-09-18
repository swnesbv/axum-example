use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use chrono::serde::ts_seconds_option;
use crate::util::date_config::date_format;


#[derive(Serialize)]
pub struct Comments {
    pub id:         i32,
    pub user_id:    i32,
    pub comment_on: Option<sqlx::types::JsonValue>,
    pub completed:  bool,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
}


#[derive(Deserialize, Serialize)]
pub struct FormComment {
    pub whose: String,
    pub comment: String,
}

#[derive(Deserialize, Serialize)]
pub struct CommentOn {
    pub user_id: i32,
    pub email:   String,
    pub name:    String,
    pub whose:   String,
    pub msg:     String,
}
