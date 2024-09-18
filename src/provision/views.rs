use sqlx::postgres::PgPool;
use sqlx::PgConnection;

use crate::provision::models::{AllPrD, AllPrH, BkgPrD, UpPrD};


pub async fn all_days(pool: PgPool) -> Result<Vec<AllPrD>, String> {
    let result = sqlx::query_as!(AllPrD, "SELECT * FROM provision_d")
        .fetch_all(&pool)
        .await
        .unwrap();
    Ok(result)
}

pub async fn all_hours(pool: PgPool) -> Result<Vec<AllPrH>, String> {
    let result = sqlx::query_as!(AllPrH, "SELECT * FROM provision_h")
        .fetch_all(&pool)
        .await
        .unwrap();
    Ok(result)
}


pub async fn details(pool: PgPool, prv_id: i32) -> Result<AllPrD, String> {
    let result = sqlx::query_as!(AllPrD, "SELECT * FROM provision_d WHERE id=$1", prv_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    Ok(result)
}


pub async fn creat_bkg(
    conn: &mut PgConnection,
    p: BkgPrD
) -> Result<sqlx::postgres::PgQueryResult, String> {

    let result = sqlx::query(
        "INSERT INTO booking (user_id, provision_d_id, title, description, st_date, en_date, created_at) VALUES ($1,$2,$3,$4,$5,$6,$7)"
        )
        .bind(p.user_id)
        .bind(p.provision_d_id)
        .bind(&p.title)
        .bind(&p.description)
        .bind(p.st_date)
        .bind(p.en_date)
        .bind(p.created_at)
        .execute(&mut *conn)
        .await
        .unwrap();

    Ok(result)
}


pub async fn list_update_prd(
    conn: &mut PgConnection,
    number: i32
) -> Result<UpPrD, String> {

    let result = sqlx::query_as!(
        UpPrD,
        "SELECT title, description,  st_date, en_date, updated_at FROM provision_d WHERE id=$1",
        number
    )
    .fetch_one(&mut *conn)
    .await
    .unwrap();
    Ok(result)
}

pub async fn post_update_prd(
    conn: &mut PgConnection,
    number: i32,
    p: UpPrD,
) -> Result<sqlx::postgres::PgRow, String> {

    let result = sqlx::query_as!(
        UpPrD,
        "UPDATE provision_d SET title=$2,  description=$3, st_date=$4, en_date=$5, updated_at=$6 WHERE id=$1",
        number, p.title, p.description, p.st_date, p.en_date, p.updated_at
    )
    .fetch_one(&mut *conn).await;

    match result {
        Ok(result) => Ok(result),
        Err(err) => Err(err.to_string()),
    }
}
