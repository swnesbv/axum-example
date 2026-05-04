use chrono::{NaiveDate};

use crate::{
    booking::models::{CheckListPrD},
    common::{PgPool}
};

pub async fn check_period_days(
    pool: PgPool,
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
    prv_id: i32
) -> Result<bool, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query(
        "SELECT id, user_id, title, description, st_date, en_date, s_dates, e_dates, dates, completed, created_at, updated_at FROM provision_d WHERE completed=true AND st_date <= $1 AND en_date >= $2 AND NOT daterange($1, $2, '[]') @> ANY(dates) OR dates IS NULL ORDER BY id",
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

pub async fn check_prd_list(
    pool: PgPool,
    start: Option<NaiveDate>,
    end: Option<NaiveDate>
) -> Result<Option<Vec<CheckListPrD>>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query(
        "SELECT id, user_id, title, description, st_date, en_date, s_dates, e_dates, dates, completed, created_at, updated_at FROM provision_d WHERE st_date <= $1 AND en_date >= $2 AND NOT daterange($1, $2, '[]') @> ANY(dates) OR dates IS NULL ORDER BY id",
        &[&start, &end]
    ).await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut r: Vec<CheckListPrD> = vec![];
    for i in rows {
        r.push(CheckListPrD {
            id:          i.get("id"),
            user_id:     i.get("user_id"),
            title:       i.get("title"),
            description: i.get("description"),
            st_date:     i.get("st_date"),
            en_date:     i.get("en_date"),
            s_dates:     i.get("s_dates"),
            e_dates:     i.get("e_dates"),
            dates:       i.get("dates"),
            completed:   i.get("completed"),
            created_at:  i.get("created_at"),
            updated_at:  i.get("updated_at")
        })
    }
    Ok(Some(r))
}

pub async fn check_prd_start(
    pool: PgPool,
    start: Option<NaiveDate>,
) -> Result<Option<Vec<CheckListPrD>>, Option<String>> {


    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query(
        "SELECT id, user_id, title, description, st_date, en_date, s_dates, e_dates, dates, completed, created_at, updated_at FROM provision_d WHERE st_date <= $1 OR dates IS NULL ORDER BY id",
        &[&start]
    ).await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut r: Vec<CheckListPrD> = vec![];
    for i in rows {
        r.push(CheckListPrD {
            id:          i.get("id"),
            user_id:     i.get("user_id"),
            title:       i.get("title"),
            description: i.get("description"),
            st_date:     i.get("st_date"),
            en_date:     i.get("en_date"),
            s_dates:     i.get("s_dates"),
            e_dates:     i.get("e_dates"),
            dates:       i.get("dates"),
            completed:   i.get("completed"),
            created_at:  i.get("created_at"),
            updated_at:  i.get("updated_at")
        })
    }
    Ok(Some(r))
}

pub async fn check_prd_end(
    pool: PgPool,
    end: Option<NaiveDate>
) -> Result<Option<Vec<CheckListPrD>>, Option<String>> {


    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query(
        "SELECT id, user_id, title, description, st_date, en_date, s_dates, e_dates, dates, completed, created_at, updated_at FROM provision_d WHERE en_date >= $1 OR dates IS NULL ORDER BY id",
        &[&end]
    ).await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut r: Vec<CheckListPrD> = vec![];
    for i in rows {
        r.push(CheckListPrD {
            id:          i.get("id"),
            user_id:     i.get("user_id"),
            title:       i.get("title"),
            description: i.get("description"),
            st_date:     i.get("st_date"),
            en_date:     i.get("en_date"),
            s_dates:     i.get("s_dates"),
            e_dates:     i.get("e_dates"),
            dates:       i.get("dates"),
            completed:   i.get("completed"),
            created_at:  i.get("created_at"),
            updated_at:  i.get("updated_at")
        })
    }
    Ok(Some(r))
}
