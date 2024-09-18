use sqlx::postgres::PgPool;

use chrono::Utc;
use jsonwebtoken::{TokenData};

use crate::{
    auth::models::{Claims},
    subscriptions::models::{AdditionallyJson}
};


pub async fn insert_ssc_user(
    pool: PgPool,
    user_id: i32,
    title: &str,
    description: &str,
    to_user: i32,
    token: TokenData<Claims>,
) -> bool {

    let s: AdditionallyJson = AdditionallyJson {
        email: token.claims.email,
        name: token.claims.username,
    };
    let str_msg = serde_json::to_string(&s).unwrap();
    let additionally: serde_json::Value = serde_json::from_str(&str_msg).unwrap();

    let result =
        sqlx::query(
            "INSERT INTO subscriptions (user_id, title, description, to_user, additionally, created_at) VALUES ($1,$2,$3,$4,$5,$6)"
        )
        .bind(user_id)
        .bind(title)
        .bind(description)
        .bind(to_user)
        .bind(additionally)
        .bind(Utc::now())
        .execute(&pool)
        .await;

    match result {
        Err(e) => {
            println!("Err..! INSERT ssc user");
            println!("Error message: [{}].\n", e);
            return false;
        }
        Ok(o) => o,
    };
    true
}

pub async fn resolution_user(
    pool: PgPool,
    id: i32,
    dialogue: String,
) -> bool {

    let result =
        sqlx::query(
            "UPDATE subscriptions SET dialogue=$2, completed=$3, updated_at=$4 WHERE id=$1"
        )
        .bind(id)
        .bind(dialogue)
        .bind(true)
        .bind(Some(Utc::now()))
        .execute(&pool)
        .await;

    match result {
        Err(e) => {
            println!("Err..! INSERT resolution user");
            println!("Error message: [{}].\n", e);
            return false;
        }
        Ok(o) => o,
    };
    true
}


pub async fn insert_ssc_group(
    pool: PgPool,
    user_id: i32,
    title: &str,
    description: &str,
    to_group: i32,
    token: TokenData<Claims>,
) -> bool {

    let s: AdditionallyJson = AdditionallyJson {
        email: token.claims.email,
        name: token.claims.username,
    };
    let str_msg = serde_json::to_string(&s).unwrap();
    let additionally: serde_json::Value = serde_json::from_str(&str_msg).unwrap();

    let dialogue = vec![user_id];
    let result =
        sqlx::query(
            "INSERT INTO subscriptions (user_id, title, description, to_group, dialogue, additionally, created_at) VALUES ($1,$2,$3,$4,$5,$6,$7)"
        )
        .bind(user_id)
        .bind(title)
        .bind(description)
        .bind(to_group)
        .bind(dialogue)
        .bind(additionally)
        .bind(Utc::now())
        .execute(&pool)
        .await;

    match result {
        Err(e) => {
            println!("Err..! INSERT ssc group");
            println!("Error message: [{}].\n", e);
            return false;
        }
        Ok(o) => o,
    };
    true
}

pub async fn resolution_group(
    pool: PgPool,
    id: i32,
    to_group: i32,
) -> bool {

    let result =
        sqlx::query(
            "UPDATE subscriptions SET dialogue=ARRAY_APPEND(dialogue, $2), completed=$3, updated_at=$4 WHERE id=$1"
        )
        .bind(id)
        .bind(to_group)
        .bind(true)
        .bind(Some(Utc::now()))
        .execute(&pool)
        .await;

    match result {
        Err(e) => {
            println!("Err..! INSERT resolution group");
            println!("Error message: [{}].\n", e);
            return false;
        }
        Ok(o) => o,
    };
    true
}

