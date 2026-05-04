use std::{collections::{HashMap, HashSet}, sync::Mutex};
use serde::{Deserialize, Serialize};
use axum::{
    http::header::{HeaderMap},
};
use tokio::sync::broadcast;
use chrono::{DateTime, Utc};

use crate::{
    util::date_config::date_format,
    common::{PgPool, RedisPool},
    auth::models::{AuToken},
    auth::check::in_check
};

#[derive(Debug)]
pub struct RoomChat {
    pub rooms: Mutex<HashMap<String, RoomState>>,
    pub pool:  PgPool,
    pub conn:  RedisPool
}
impl RoomChat {
    pub async fn ctx(
        &self, headers: HeaderMap
    ) -> Result<Option<AuToken>, Option<String>> {
        let conn = &self.conn;
        match in_check(conn.clone(), headers).await {
            Ok(expr) => Ok(expr),
            Err(_) => Ok(None),
        }
    }
}
#[derive(Debug)]
pub struct RoomState {
    pub user_set: HashSet<String>,
    pub tx:       broadcast::Sender<String>,
}

#[allow(clippy::all)]
impl RoomState {
    pub fn new() -> Self {
        Self {
            user_set: HashSet::new(),
            tx:       broadcast::channel(100).0,
        }
    }
}

pub struct UserChat {
    pub user_set: Mutex<HashSet<String>>,
    pub tx:       broadcast::Sender<String>,
    pub pool:     PgPool,
    pub conn:     RedisPool
}
impl UserChat {
    pub async fn ctx(
        &self, headers: HeaderMap
    ) -> Result<Option<AuToken>, Option<String>> {
        let conn = &self.conn;
        match in_check(conn.clone(), headers).await {
            Ok(expr) => Ok(expr),
            Err(_) => Ok(None),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Connect {
    pub username: String,
    pub channel:  String,
}

#[derive(Deserialize, Serialize)]
pub struct Msg {
    pub id:  String,
    pub txt: String,
}
#[derive(Deserialize, Serialize)]
pub struct InOut {
    pub txt: String,
}

#[derive(Serialize, Deserialize)]
pub struct PublicChat {
	pub id:         i32,
	pub user_id:    i32,
	pub joined:     Option<String>,
	pub came_out:   Option<String>,
	pub message:    Option<String>,
	#[serde(with = "date_format")]
	pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Room {
    pub id:         i32,
    pub user_id:    i32,
    pub joined:     Option<String>,
    pub came_out:   Option<String>,
    pub message:    Option<String>,
    pub room:       String,
	#[serde(with = "date_format")]
	pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DialogueChat {
    pub user_id: i32,
    pub to_user: Option<i32>,
}


#[derive(Deserialize, Serialize)]
pub struct FormDel {
	pub to_del: Vec<i32>,
	pub on_off: Vec<String>,
}


#[derive(Deserialize, Serialize)]
pub struct GetParam {
  pub page: Option<String>
}