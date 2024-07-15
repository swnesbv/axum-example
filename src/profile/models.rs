use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use axum::{
    body::Body,
    http::{Response},
};

use crate::util::date_config::date_format;
use chrono::serde::ts_seconds_option;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NaUser {
    pub username: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmUser {
    pub email: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub password: String,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ListUser {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub img: Option<String>,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormNewUser {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateUser {
    pub email: String,
    pub username: String,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormUpdateUser {
    pub email: String,
    pub username: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PasswordChange {
    pub password: String,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormPasswordChange {
    pub password: String,
}

pub enum EnumError{
    ResBody(Response<Body>),
    ErrString(String)
}
