use sqlx::postgres::PgPool;

use chrono::{
    Utc, NaiveDateTime,
};

use crate::{
    schedule::models::{
        Schedule, Places, Recording
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

pub async fn all_rec(
    pool: PgPool,
) -> Result<Vec<Recording>, String> {

    let result = sqlx::query_as!(
        Recording, "SELECT * FROM recording"
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


pub async fn int_selection(
    a: Vec<i32>,
    b: Vec<i32>,
) -> Vec<i32> {

    let mut c = Vec::new();
    for i in a {
        if !b.contains(&i) {
            c.push(i);
        }
    }
    c
}

pub async fn places_select(
    pool: PgPool,
) -> Result<Vec<Places>, String> {

    let mut result = sqlx::query_as!(
        Places,
        "SELECT id, user_id, title, description, hours, places, non_places, completed, created_at, updated_at FROM schedule  WHERE CURRENT_TIMESTAMP <= ANY (hours)"
        )
        .fetch_all(&pool).await.unwrap();
    
    for i in &mut result {
        if i.non_places.is_some() {
            let free = int_selection(i.places.as_ref().expect("REASON").to_vec(), i.non_places.as_ref().expect("REASON").to_vec()).await;
            println!("{:?}", free);
            i.places = Some(free);
            return Ok(result)
        }
    }
    Ok(result)
}
