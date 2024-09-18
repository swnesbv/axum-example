use sqlx::postgres::PgPool;

// use chrono::NaiveDate;

use crate::{auth::models::ListUser, import_export::models::CsvUser};

use csv::Writer;
use std::error::Error;

pub async fn all(pool: PgPool) -> Result<Vec<CsvUser>, String> {
    let result = sqlx::query_as!(CsvUser, "SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .unwrap();
    Ok(result)
}

pub async fn write_to_csv(data: Vec<ListUser>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_writer(vec![]);

    for pat in data {
        wtr.serialize(pat)?;
    }

    wtr.flush()?;

    Ok(())
}
