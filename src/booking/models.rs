use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::util::date_config::date_format;
use chrono::serde::ts_seconds_option;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::booking)]
pub struct NewBooking {
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormNewBooking {
    pub title: String,
    pub description: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormSE {
    pub start: NaiveDate,
    pub end: NaiveDate,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Claims {
    pub start: NaiveDate,
    pub end: NaiveDate,
}


#[derive(AsChangeset, Identifiable, Queryable, PartialEq, Debug, Clone, Selectable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::booking)]
pub struct LtBkg {
    pub id: i32,
    pub user_id: i32,
    pub provision_d_id: Option<i32>,
    pub provision_h_id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub st_date: Option<NaiveDate>,
    pub en_date: Option<NaiveDate>,
    pub st_hour: Option<NaiveDateTime>,
    pub en_hour: Option<NaiveDateTime>,
    pub completed: bool,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
}


use diesel::deserialize::QueryableByName;

#[derive(QueryableByName, AsChangeset, Identifiable, Queryable, PartialEq, Debug, Clone, Selectable, Serialize)]
#[diesel(table_name = crate::schema::provision_d)]
pub struct SqlPrD {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub st_date: Option<NaiveDate>,
    pub en_date: Option<NaiveDate>,
    pub s_dates: Option<Vec<Option<NaiveDate>>>,
    pub e_dates: Option<Vec<Option<NaiveDate>>>,
    pub dates: Option<Vec<Option<NaiveDate>>>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}


#[derive(AsChangeset, Identifiable, Queryable, PartialEq, Debug, Clone, Selectable, Serialize)]
#[diesel(table_name = crate::schema::provision_d)]
pub struct AllPrD {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub st_date: Option<NaiveDate>,
    pub en_date: Option<NaiveDate>,
    pub s_dates: Option<Vec<Option<NaiveDate>>>,
    pub e_dates: Option<Vec<Option<NaiveDate>>>,
    pub dates: Option<Vec<Option<NaiveDate>>>,
    pub completed: bool,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
}
