use crate::{
    common::{PgPool},
    provision::models::{UpPrD, BkgPrD}
};

pub async fn one_update_prd(
    pool: PgPool,
    number: i32
) -> Result<UpPrD, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query_one(
        "SELECT title, description,  st_date, en_date, completed, updated_at FROM provision_d WHERE id=$1",
        &[&number]
    ).await;
    let i = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let r: UpPrD = UpPrD {
        title:        i.get("title"),
        description:  i.get("description"),
        st_date:      i.get("st_date"),
        en_date:      i.get("en_date"),
        completed:    i.get("completed"),
    };
    Ok(r)
}

pub async fn post_update_prd(
    pool: PgPool,
    p: UpPrD,
    number: i32,
) -> Result<u64, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result =
        pg.execute(
        "UPDATE provision_d SET title=$2,  description=$3, st_date=$4, en_date=$5, completed=$6, updated_at=now() WHERE id=$1",
        &[&number, &p.title, &p.description, &p.st_date, &p.en_date, &p.completed]
    ).await;
    let r = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    Ok(r)
}

pub async fn del_prd(
    pool: PgPool,
    id: i32, user_id: i32
) -> Result<u64, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.execute(
        "DELETE FROM provision_d WHERE id=$1 AND user_id=$2",
        &[&id, &user_id]
    )
    .await;
    match result {
        Ok(expr) => Ok(expr),
        Err(err) => Err(Some(err.to_string()))
    }
}


pub async fn creat_bkg_days(
    pool: PgPool,
    b: BkgPrD,
) -> Result<u64, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.execute(
        "INSERT INTO booking (user_id, provision_d_id, title, description, st_date, en_date, created_at) VALUES ($1,$2,$3,$4,$5,$6,now())",
        &[&b.user_id, &b.provision_d_id, &b.title, &b.description, &b.st_date, &b.en_date]
        ).await;
    let r = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    Ok(r)
}