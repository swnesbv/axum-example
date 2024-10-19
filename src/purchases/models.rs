use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use chrono::serde::ts_seconds_option;
use crate::util::date_config::date_format;


#[derive(Serialize)]
pub struct Purchases {
    pub id:         i32,
    pub user_id:    i32,
    pub product_id: i32,
    pub categories: Option<Vec<String>>,
    pub amount:     Option<sqlx::types::JsonValue>,
    pub price:      Option<sqlx::types::JsonValue>,
    pub completed:  bool,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct PurchasesCls {
    pub user_id:    i32,
    pub product_id: i32,
    pub amount:     Option<sqlx::types::JsonValue>,
    pub price:      Option<sqlx::types::JsonValue>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FormPurchases {
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

    pub product_id: i32,
}
// #[derive(Debug, Deserialize, Serialize)]
// pub struct FormSelect {
//     pub categories: Vec<String>,
//     pub on_off:     Vec<String>,
// }
