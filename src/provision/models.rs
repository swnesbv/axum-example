use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::util::date_config::date_format;
use chrono::serde::ts_seconds_option;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::provision_d)]
pub struct NewPrD {
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub st_date: Option<NaiveDate>,
    pub en_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
}
#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::provision_h)]
pub struct NewPrH {
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub st_hour: Option<NaiveDateTime>,
    pub en_hour: Option<NaiveDateTime>,
    pub created_at: DateTime<Utc>,
}

#[derive(AsChangeset, Identifiable, Queryable, PartialEq, Debug, Clone, Selectable, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormPrD {
    pub title: String,
    pub description: Option<String>,
    pub st_date: Option<String>,
    pub en_date: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormPrH {
    pub title: String,
    pub description: Option<String>,
    pub st_hour: Option<String>,
    pub en_hour: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormPrdBkg {
    pub title: String,
    pub description: Option<String>,
    pub s_dates: Option<NaiveDate>,
    pub e_dates: Option<NaiveDate>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::provision_d)]
pub struct UpPrD {
    pub title: String,
    pub description: Option<String>,
    pub st_date: Option<NaiveDate>,
    pub en_date: Option<NaiveDate>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
}


#[derive(Queryable, PartialEq, Debug, Clone, Deserialize, Serialize, Selectable, Insertable)]
#[diesel(table_name = crate::schema::booking)]
pub struct BkgPrD {
    pub user_id: i32,
    pub provision_d_id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub st_date: Option<NaiveDate>,
    pub en_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::provision_d)]
pub struct UpPrdBkg {
    pub s_dates: Option<Vec<Option<NaiveDate>>>,
    pub e_dates: Option<Vec<Option<NaiveDate>>>,
    pub dates: Option<Vec<Option<NaiveDate>>>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
}
