use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use chrono::serde::ts_seconds_option;
use crate::util::date_config::date_format;


#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FormComment {
    pub to_id:   Option<i32>,
    pub len_cmt: i32,
    pub comment: Option<String>
}
#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd, Ord, Eq)]
pub struct UpCmt {
    pub msg:        String,
    pub completed:  bool,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>
}
#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd, Ord, Eq)]
pub struct Cmt {
    pub id:         String,
    pub user_id:    i32,
    pub tab_id:     i32,
    pub email:      String,
    pub name:       String,
    pub msg:        String,
    pub completed:  bool,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Comment {
    pub comments: Option<Vec<Value>>
}
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct VecCmt {
    pub comments: Vec<Cmt>
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FormUpdateCmt {
    pub msg:       Option<String>,
    pub completed: Option<String>
}