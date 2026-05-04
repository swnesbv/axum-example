use crate::{
    common::{PgPool},
    provision::models::{UpPrH, BkgPrH}
};

pub async fn one_update_prh(
    pool: PgPool,
    number: i32
) -> Result<UpPrH, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query_one(
        "SELECT title,description,st_hour,en_hour,completed,updated_at FROM provision_h WHERE id=$1",
        &[&number]
    ).await;
    let i = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let r: UpPrH = UpPrH {
        title:        i.get("title"),
        description:  i.get("description"),
        st_hour:      i.get("st_hour"),
        en_hour:      i.get("en_hour"),
        completed:    i.get("completed"),
    };
    Ok(r)
}

pub async fn post_update_prh(
    pool: PgPool,
    p: UpPrH,
    number: i32,
) -> Result<u64, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result =
        pg.execute(
        "UPDATE provision_h SET title=$2,  description=$3, st_hour=$4, en_hour=$5, completed=$6, updated_at=now() WHERE id=$1",
        &[&number, &p.title, &p.description, &p.st_hour, &p.en_hour, &p.completed]
    ).await;
    let r = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    Ok(r)
}

pub async fn del_prh(
    pool: PgPool,
    id: i32, user_id: i32
) -> Result<u64, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.execute(
        "DELETE FROM provision_h WHERE id=$1 AND user_id=$2",
        &[&id, &user_id]
    )
    .await;
    match result {
        Ok(expr) => Ok(expr),
        Err(err) => Err(Some(err.to_string()))
    }
}

pub async fn creat_bkg_hours(
    pool: PgPool,
    b: BkgPrH,
) -> Result<u64, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.execute(
        "INSERT INTO booking (user_id, provision_h_id, title, description, st_hour, en_hour, created_at) VALUES ($1,$2,$3,$4,$5,$6,now())",
        &[&b.user_id, &b.provision_h_id, &b.title, &b.description, &b.st_hour, &b.en_hour]
        ).await;
    let r = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    Ok(r)
}