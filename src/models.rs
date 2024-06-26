use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use diesel::prelude::*;

// use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, PartialEq, Debug, Clone, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub img: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Clone, Selectable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = crate::schema::article)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Article {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub img: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Clone, Selectable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = crate::schema::provision_d)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProvisionD {
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

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Clone, Selectable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = crate::schema::provision_h)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProvisionH {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub st_hour: Option<NaiveDateTime>,
    pub en_hour: Option<NaiveDateTime>,
    pub s_hours: Option<Vec<Option<NaiveDateTime>>>,
    pub e_hours: Option<Vec<Option<NaiveDateTime>>>,
    pub hours: Option<Vec<Option<NaiveDateTime>>>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Clone, Selectable)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(ProvisionD))]
#[diesel(belongs_to(ProvisionH))]
#[diesel(table_name = crate::schema::booking)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Booking {
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
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Sessions {
    pub session_token: Vec<u8>,
    pub id: Option<i32>,
}
