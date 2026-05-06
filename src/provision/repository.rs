use crate::{
    common::{PgPool},
    provision::models::{AllPrD, AllPrH}
};


pub async fn all_days(
    pool: PgPool
) -> Result<Vec<AllPrD>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result =  pg.query("SELECT * FROM provision_d;", &[]).await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut r: Vec<AllPrD> = vec![];
    for i in rows {
        r.push(AllPrD {
            id:           i.get("id"),
            user_id:      i.get("user_id"),
            title:        i.get("title"),
            description:  i.get("description"),
            st_date:      i.get("st_date"),
            en_date:      i.get("en_date"),
            s_dates:      i.get("s_dates"),
            e_dates:      i.get("e_dates"),
            dates:        i.get("dates"),
            completed:    i.get("completed"),
            created_at:   i.get("created_at"),
            updated_at:   i.get("updated_at")
        })
    }
    Ok(r)
}

pub async fn all_hours(
    pool: PgPool
) -> Result<Vec<AllPrH>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query("SELECT * FROM provision_h;", &[]).await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut r: Vec<AllPrH> = vec![];
    for i in rows {
        r.push(AllPrH {
            id:           i.get("id"),
            user_id:      i.get("user_id"),
            title:        i.get("title"),
            description:  i.get("description"),
            st_hour:      i.get("st_hour"),
            en_hour:      i.get("en_hour"),
            s_hours:      i.get("s_hours"),
            e_hours:      i.get("e_hours"),
            hours:        i.get("hours"),
            completed:    i.get("completed"),
            created_at:   i.get("created_at"),
            updated_at:   i.get("updated_at")
        })
    }
    Ok(r)
}

pub async fn details_prd(
    pool: PgPool, prv_id: i32
) -> Result<AllPrD, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query_one(
        "SELECT * FROM provision_d WHERE id=$1;",
    &[&prv_id]
    ).await;
    let i = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let r: AllPrD = AllPrD {
        id:           i.get("id"),
        user_id:      i.get("user_id"),
        title:        i.get("title"),
        description:  i.get("description"),
        st_date:      i.get("st_date"),
        en_date:      i.get("en_date"),
        s_dates:      i.get("s_dates"),
        e_dates:      i.get("e_dates"),
        dates:        i.get("dates"),
        completed:    i.get("completed"),
        created_at:   i.get("created_at"),
        updated_at:   i.get("updated_at")
    };
    Ok(r)
}

pub async fn details_prh(
    pool: PgPool, prv_id: i32
) -> Result<AllPrH, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query_one(
        "SELECT * FROM provision_h WHERE id=$1;",
    &[&prv_id]
    ).await;
    let i = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let r: AllPrH = AllPrH {
        id:           i.get("id"),
        user_id:      i.get("user_id"),
        title:        i.get("title"),
        description:  i.get("description"),
        st_hour:      i.get("st_hour"),
        en_hour:      i.get("en_hour"),
        s_hours:      i.get("s_hours"),
        e_hours:      i.get("e_hours"),
        hours:        i.get("hours"),
        completed:    i.get("completed"),
        created_at:   i.get("created_at"),
        updated_at:   i.get("updated_at")
    };
    Ok(r)
}