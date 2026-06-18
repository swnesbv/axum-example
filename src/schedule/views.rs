use chrono::{NaiveDateTime, Utc};

use crate::{
    common::{PgPool},
    schedule::models::{Places, Recording, Schedule, Title}
};

pub async fn all_sch(
    pool: PgPool
) -> Result<Vec<Schedule>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query("SELECT * FROM schedule;", &[]).await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut r: Vec<Schedule> = vec![];
    rows.iter().for_each(|i| {
        r.push(Schedule {
            id:          i.get("id"),
            user_id:     i.get("user_id"),
            title:       i.get("title"),
            description: i.get("description"),
            st_hour:     i.get("st_hour"),
            en_hour:     i.get("en_hour"),
            hours:       i.get("hours"),
            occupied:    i.get("occupied"),
            places:      i.get("places"),
            non_places:  i.get("non_places"),
            completed:   i.get("completed"),
            created_at:  i.get("created_at"),
            updated_at:  i.get("updated_at")
        })
    });
    Ok(r)
}

pub async fn details(
    pool: PgPool,
    to_schedule: i32
) -> Result<String, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query_one(
        "SELECT title FROM schedule WHERE id=$1", &[&to_schedule]
    ).await;
    let i = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let r: Title = Title {
        title: i.get("title"),
    };
    Ok(r.title)
}

pub async fn all_rec(
    pool: PgPool
) -> Result<Vec<Recording>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query("SELECT * FROM recording", &[]).await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut r: Vec<Recording> = vec![];
    rows.iter().for_each(|i| {
        r.push(Recording {
            id:          i.get("id"),
            user_id:     i.get("user_id"),
            to_schedule: i.get("to_schedule"),
            record_d:    i.get("record_d"),
            record_h:    i.get("record_h"),
            places:      i.get("en_hour"),
            tickets:     i.get("hours"),
            completed:   i.get("completed"),
            created_at:  i.get("created_at"),
            updated_at:  i.get("updated_at")
        })
    });
    Ok(r)
}

pub async fn selection(
    a: Vec<NaiveDateTime>,
    b: Vec<NaiveDateTime>
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
    pool: PgPool
) -> Result<Vec<Schedule>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query(
        "SELECT * FROM schedule WHERE en_hour >= $1",
        &[&Utc::now().naive_utc()]
    ).await;

    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut r: Vec<Schedule> = vec![];
    rows.iter().for_each(|i| {
        r.push(Schedule {
            id:          i.get("id"),
            user_id:     i.get("user_id"),
            title:       i.get("title"),
            description: i.get("description"),
            st_hour:     i.get("st_hour"),
            en_hour:     i.get("en_hour"),
            hours:       i.get("hours"),
            occupied:    i.get("occupied"),
            places:      i.get("places"),
            non_places:  i.get("non_places"),
            completed:   i.get("completed"),
            created_at:  i.get("created_at"),
            updated_at:  i.get("updated_at")
        })
    });

    for i in &mut r {
        if let Some(item) = &i.occupied {
            let free = selection(
                i.hours.as_ref().unwrap().to_vec(),
                item.to_vec(),
            )
            .await;
            println!("{:?}", free);
            i.hours = Some(free);
            return Ok(r);
        }
    }
    Ok(r)
}

pub async fn int_selection(
    a: Vec<i32>, b: Vec<i32>
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
    pool: PgPool
) -> Result<Vec<Places>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query(
        "SELECT id, user_id, title, description, hours, places, non_places, completed, created_at, updated_at FROM schedule  WHERE CURRENT_TIMESTAMP <= ANY (hours)", &[]
    ).await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut r: Vec<Places> = vec![];
    rows.iter().for_each(|i| {
        r.push(Places {
            id:          i.get("id"),
            user_id:     i.get("user_id"),
            title:       i.get("title"),
            description: i.get("description"),
            hours:       i.get("hours"),
            places:      i.get("places"),
            non_places:  i.get("non_places"),
            completed:   i.get("completed"),
            created_at:  i.get("created_at"),
            updated_at:  i.get("updated_at")
        })
    });
    for i in &mut r {
        if let Some(item) = &i.non_places {
            let free = int_selection(
                i.places.as_ref().expect("REASON").to_vec(),
                item.to_vec(),
            ).await;
            println!("{:?}", free);
            i.places = Some(free);
            return Ok(r);
        }
    }
    Ok(r)
}
