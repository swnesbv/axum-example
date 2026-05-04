use std::fmt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use chrono::serde::ts_seconds_option;

use crate::util::date_config::date_format;


pub struct SliceDisplay<'a, T: 'a>(pub &'a [T]);
impl<'a, T: fmt::Display + 'a> fmt::Display for SliceDisplay<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        for item in self.0 {
            if !first {
                write!(f, ", {}", item)?;
            } else {
                write!(f, "{}", item)?;
            }
            first = false;
        }
        Ok(())
    }
}


#[derive(Debug, Deserialize, Serialize, Default)]
pub struct CsvUser {
	pub id:       	i32,
	pub email: 	  	String,
	pub username: 	String,
	pub password: 	String,
	pub img: 	  	Option<String>,
	pub status:   	Vec<String>,
	#[serde(with = "date_format")]
	pub created_at: DateTime<Utc>,
	#[serde(with = "ts_seconds_option")]
	pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct ExCsvUser {
	pub email:    String,
	pub username: String,
	pub password: String,
	pub img: 	  Option<String>,
	pub status:   String,
}
