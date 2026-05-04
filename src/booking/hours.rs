use chrono::{NaiveDateTime};

use crate::{
    booking::models::{CheckListPrH},
    common::{PgPool}
};

pub async fn check_period_hours(
    pool: PgPool,
    start: Option<NaiveDateTime>,
    end: Option<NaiveDateTime>,
    prv_id: i32
) -> Result<bool, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query(
        "SELECT id, user_id, title, description, st_hour, en_hour, s_hours, e_hours, hours, completed, created_at, updated_at FROM provision_h WHERE completed=true AND st_hour <= $1 AND en_hour >= $2 AND NOT tsrange($1, $2, '[]') @> ANY(hours) OR hours IS NULL ORDER BY id",
        &[&start, &end]
    ).await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    for i in rows {
        if i.get::<&str, i32>("id") == prv_id {
            return Ok(true)
        }
    }
    Ok(false)
}

pub async fn check_prh_list(
    pool: PgPool,
    start: Option<NaiveDateTime>,
    end: Option<NaiveDateTime>
) -> Result<Option<Vec<CheckListPrH>>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query(
        "SELECT id, user_id, title, description, st_hour, en_hour, s_hours, e_hours, hours, completed, created_at, updated_at FROM provision_h WHERE st_hour <= $1 AND en_hour >= $2 AND NOT tsrange($1, $2, '[]') @> ANY(hours) OR hours IS NULL ORDER BY id",
        &[&start, &end]
    ).await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut r: Vec<CheckListPrH> = vec![];
    for i in rows {
        r.push(CheckListPrH {
            id:          i.get("id"),
            user_id:     i.get("user_id"),
            title:       i.get("title"),
            description: i.get("description"),
            st_hour:     i.get("st_hour"),
            en_hour:     i.get("en_hour"),
            s_hours:     i.get("s_hours"),
            e_hours:     i.get("e_hours"),
            hours:       i.get("hours"),
            completed:   i.get("completed"),
            created_at:  i.get("created_at"),
            updated_at:  i.get("updated_at")
        })
    }
    Ok(Some(r))
}

pub async fn check_prh_start(
    pool: PgPool,
    start: Option<NaiveDateTime>,
) -> Result<Option<Vec<CheckListPrH>>, Option<String>> {


    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query(
        "SELECT id, user_id, title, description, st_date, en_date, s_dates, e_hours, hours, completed, created_at, updated_at FROM provision_h WHERE st_hour <= $1 OR hours IS NULL ORDER BY id",
        &[&start]
    ).await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut r: Vec<CheckListPrH> = vec![];
    for i in rows {
        r.push(CheckListPrH {
            id:          i.get("id"),
            user_id:     i.get("user_id"),
            title:       i.get("title"),
            description: i.get("description"),
            st_hour:     i.get("st_hour"),
            en_hour:     i.get("en_hour"),
            s_hours:     i.get("s_hours"),
            e_hours:     i.get("e_hours"),
            hours:       i.get("hours"),
            completed:   i.get("completed"),
            created_at:  i.get("created_at"),
            updated_at:  i.get("updated_at")
        })
    }
    Ok(Some(r))
}

pub async fn check_prh_end(
    pool: PgPool,
    end: Option<NaiveDateTime>
) -> Result<Option<Vec<CheckListPrH>>, Option<String>> {


    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query(
        "SELECT id, user_id, title, description, st_hour, en_hour, s_hours, e_hours, hours, completed, created_at, updated_at FROM provision_h WHERE en_hour >= $1 OR hours IS NULL ORDER BY id",
        &[&end]
    ).await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut r: Vec<CheckListPrH> = vec![];
    for i in rows {
        r.push(CheckListPrH {
            id:          i.get("id"),
            user_id:     i.get("user_id"),
            title:       i.get("title"),
            description: i.get("description"),
            st_hour:     i.get("st_hour"),
            en_hour:     i.get("en_hour"),
            s_hours:     i.get("s_hours"),
            e_hours:     i.get("e_hours"),
            hours:       i.get("hours"),
            completed:   i.get("completed"),
            created_at:  i.get("created_at"),
            updated_at:  i.get("updated_at")
        })
    }
    Ok(Some(r))
}
