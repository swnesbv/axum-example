use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::util::date_config::date_format;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImgUser {
    pub img: String,
    #[serde(with = "date_format")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Msg {
    pub msg: String,
    pub alert: String,
}