use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::util::date_config::date_format;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
pub struct ImgUser {
    pub img: String,
    #[serde(with = "date_format")]
    pub updated_at: DateTime<Utc>,
}
