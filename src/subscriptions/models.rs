use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::util::date_config::date_format;
use chrono::serde::ts_seconds_option;


#[derive(Serialize)]
pub struct Additionally {
	pub additionally: sqlx::types::JsonValue,
}
#[derive(Deserialize, Serialize)]
pub struct AdditionallyJson {
    pub email:   String,
    pub name:    String,
}

#[derive(Serialize)]
pub struct ToDialogue {
	pub to_user:      Option<i32>,
	pub additionally: sqlx::types::JsonValue,
    pub completed:    bool,
}


#[derive(Serialize)]
pub struct Subscription {
	pub id:           i32,
	pub user_id:      i32,
	pub title:        String,
	pub description:  Option<String>,
	pub to_user:      Option<i32>,
	pub to_group:     Option<i32>,
	pub dialogue:     Option<String>,
	pub additionally: sqlx::types::JsonValue,
    pub completed:    bool,
    #[serde(with = "date_format")]
    pub created_at:   DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at:   Option<DateTime<Utc>>,
}


#[derive(Deserialize, Serialize)]
pub struct FormGroup {
	pub title: 		 String,
	pub description: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct FormSsc {
	pub to_user:  	 Option<i32>,
	pub to_group: 	 Option<i32>,
}

#[derive(Deserialize, Serialize)]
pub struct FormResolution {
	pub id:          i32,
	pub to_user:  	 Option<i32>,
	pub to_group: 	 Option<i32>,
}


#[derive(Serialize)]
pub struct Group {
	pub id:          i32,
	pub user_id:     i32,
	pub title:       String,
	pub description: Option<String>,
	pub img:     	 Option<String>,
    pub completed:   bool,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
}