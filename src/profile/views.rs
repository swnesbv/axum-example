use sqlx::postgres::PgPool;

// use chrono::NaiveDate;

use crate::{
    profile::models::{
    // EnumError,
    ListUser,
    UpdateUser,
}
};


pub async fn all(pool: PgPool) -> Result<Vec<ListUser>, String> {
    let result = sqlx::query_as!(
        ListUser,
        "SELECT id, email, username, img, created_at, updated_at FROM users"
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    Ok(result)
}


pub async fn details(
    conn: &mut sqlx::PgConnection,
    name: String
) -> Result<ListUser, String> {

    let result = sqlx::query_as!(
        ListUser,
        "SELECT id, email, username, img, created_at, updated_at FROM users WHERE username=$1",
        name
    )
    .fetch_one(&mut *conn)
    .await;
    match result {
        Ok(result) => Ok(result),
        Err(err) => Err(err.to_string()),
    }
}


pub async fn update_details(pool: PgPool, id: i32) -> Result<UpdateUser, String> {
    let result = sqlx::query_as!(
        UpdateUser,
        "SELECT email, username, updated_at FROM users WHERE id=$1",
        id
    )
    .fetch_one(&pool)
    .await;
    match result {
        Ok(result) => Ok(result),
        Err(err) => Err(err.to_string()),
    }
}
