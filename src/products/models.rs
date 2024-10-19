use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use chrono::serde::ts_seconds_option;
use crate::util::date_config::date_format;


#[derive(Serialize)]
pub struct Categories {
    pub object_1: Option<String>,
    pub object_2: Option<String>,
    pub object_3: Option<String>,
    pub object_4: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AmountPrice {
    pub container: Option<i32>,
    pub boxes:     Option<i32>,
    pub carton:    Option<i32>,
    pub units:     Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct Products {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub categories: Option<Vec<String>>,
    pub amount:    Option<sqlx::types::JsonValue>,
    pub price:    Option<sqlx::types::JsonValue>,
    pub img:     Option<String>,
    pub completed: bool,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct FormProducts {
    pub title:       String,
    pub description: Option<String>,
    pub amount:      Option<String>,
    pub price:       Option<String>,

    pub a_container: Option<i32>,
    pub a_boxes:     Option<i32>,
    pub a_carton:    Option<i32>,
    pub a_units:     Option<i32>,

    pub p_container: Option<i32>,
    pub p_boxes:     Option<i32>,
    pub p_carton:    Option<i32>,
    pub p_units:     Option<i32>,

    pub categories:  Vec<String>,
    pub on_off:      Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FormSelect {
    pub categories: Vec<String>,
    pub on_off:     Vec<String>,
}
