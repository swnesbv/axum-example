use sqlx::postgres::PgPool;

use chrono::{
    Utc, NaiveDateTime,
};

use crate::{
    schedule::models::{
        Schedule,
    },
};


pub async fn all_sch(
    pool: PgPool,
) -> Result<Vec<Schedule>, String> {

    let result = sqlx::query_as!(
        Schedule, "SELECT * FROM schedule"
        )
        .fetch_all(&pool).await.unwrap();
    Ok(result)
}

pub async fn selection(
    a: Vec<NaiveDateTime>,
    b: Vec<NaiveDateTime>,
) -> Vec<NaiveDateTime> {

    let mut c = Vec::new();
    for i in a {
        if !b.contains(&i) {
            c.push(i);
        }
    }
    c
}


pub async fn sch_select(
    pool: PgPool,
) -> Result<Vec<Schedule>, String> {

    let mut result = sqlx::query_as!(
        Schedule,
        "SELECT * FROM schedule WHERE en_hour >= $1", Utc::now().naive_utc()
        )
        .fetch_all(&pool).await.unwrap();
    
    for i in &mut result {
        if i.occupied.is_some() {
            let free = selection(i.hours.as_ref().expect("REASON").to_vec(), i.occupied.as_ref().expect("REASON").to_vec()).await;
            println!("{:?}", free);
            i.hours = Some(free);
            return Ok(result)
        }
    }
    Ok(result)
}
