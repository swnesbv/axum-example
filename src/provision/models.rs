use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::util::date_config::date_format;
use chrono::serde::ts_seconds_option;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Prd {
    pub title:       Option<String>,
    pub description: Option<String>,
    pub s_dates:     Option<NaiveDate>,
    pub e_dates:     Option<NaiveDate>,
    pub to_id:       Option<i32>,
    pub comment:     Option<String>
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Prh {
    pub title:       Option<String>,
    pub description: Option<String>,
    pub s_hours:     Option<NaiveDateTime>,
    pub e_hours:     Option<NaiveDateTime>,
    pub to_id:       Option<i32>,
    pub comment:     Option<String>
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MyType {
    pub d: Prd,
    pub h: Prh
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParsePointError;

#[derive(Deserialize, Serialize)]
pub struct AllPrD {
    pub id:          i32,
    pub user_id:     i32,
    pub title:       String,
    pub description: Option<String>,
    pub st_date:     Option<NaiveDate>,
    pub en_date:     Option<NaiveDate>,
    pub s_dates:     Option<Vec<NaiveDate>>,
    pub e_dates:     Option<Vec<NaiveDate>>,
    pub dates:       Option<Vec<NaiveDate>>,
    pub completed:   bool,
    #[serde(with = "date_format")]
    pub created_at:  DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at:  Option<DateTime<Utc>>,
}
#[derive(Deserialize, Serialize)]
pub struct AllPrH {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub st_hour: Option<NaiveDateTime>,
    pub en_hour: Option<NaiveDateTime>,
    pub s_hours: Option<Vec<NaiveDateTime>>,
    pub e_hours: Option<Vec<NaiveDateTime>>,
    pub hours: Option<Vec<NaiveDateTime>>,
    pub completed: bool,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
}


#[derive(Deserialize, Serialize)]
pub struct FormPrH {
    pub title: String,
    pub description: Option<String>,
    pub st_hour: Option<String>,
    pub en_hour: Option<String>,
    pub completed: Option<String>,
}

#[derive(Default, Deserialize, Serialize)]
pub struct UpPrD {
    pub title: String,
    pub description: Option<String>,
    pub st_date: Option<NaiveDate>,
    pub en_date: Option<NaiveDate>,
    pub completed: bool,
}

#[derive(Default, Deserialize, Serialize)]
pub struct UpPrH {
    pub title: String,
    pub description: Option<String>,
    pub st_hour: Option<NaiveDateTime>,
    pub en_hour: Option<NaiveDateTime>,
    pub completed: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormPrd {
    pub title: String,
    pub description: Option<String>,
    pub s_dates: NaiveDate,
    pub e_dates: NaiveDate
}
#[derive(Clone, Deserialize, Serialize)]
pub struct FormStringPrd {
    pub title: String,
    pub description: Option<String>,
    pub st_date: Option<String>,
    pub en_date: Option<String>,
    pub completed: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct FormCreatBkgD {
    pub title: String,
    pub description: Option<String>,
    pub s_dates: NaiveDate,
    pub e_dates: NaiveDate,
    pub prv_id: String,
}
#[derive(Deserialize, Serialize)]
pub struct FormCreatBkgH {
    pub title: String,
    pub description: Option<String>,
    pub s_hours: NaiveDateTime,
    pub e_hours: NaiveDateTime,
    pub prv_id: String,
}

#[derive(Serialize)]
pub struct BkgPrD {
    pub user_id: i32,
    pub provision_d_id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub st_date: Option<NaiveDate>,
    pub en_date: Option<NaiveDate>,
}

#[derive(Serialize)]
pub struct BkgPrH {
    pub user_id: i32,
    pub provision_h_id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub st_hour: Option<NaiveDateTime>,
    pub en_hour: Option<NaiveDateTime>,
}

#[derive(Serialize)]
pub struct UpPrdBkg {
    pub s_dates: Vec<NaiveDate>,
    pub e_dates: Vec<NaiveDate>,
    pub dates: Vec<NaiveDate>,
}
