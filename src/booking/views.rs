use sqlx::postgres::PgPool;

use chrono::NaiveDate;

use crate::booking::models::{LtBkg, SllPrD};


pub async fn all(pool: PgPool) -> Result<Vec<LtBkg>, String> {
    let result = sqlx::query_as!(LtBkg, "SELECT * FROM booking")
        .fetch_all(&pool)
        .await
        .unwrap();
    Ok(result)
}

pub async fn slt(
    pool: PgPool, start: NaiveDate, end: NaiveDate
) -> Result<Vec<SllPrD>, String> {

    let result = sqlx::query_as!(
        SllPrD,
        "SELECT id, user_id, title, description, st_date, en_date, s_dates, e_dates, dates, completed, created_at, updated_at FROM provision_d WHERE st_date <= $1 AND en_date >= $2 AND NOT daterange($1, $2, '[]') @> ANY(dates) OR dates IS NULL ORDER BY id",
        start, end
    )
    .fetch_all(&pool).await.unwrap();
    Ok(result)
}
