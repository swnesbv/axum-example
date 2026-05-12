use crate::{
    common::PgPool,
    //photo::models::{Collections}
};

pub async fn insert_collection(
    pool:       PgPool,
    user_id:    i32,
    to_product: i32,
    vec_img:    serde_json::Value,
) -> Result<u64, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.execute(
        "INSERT INTO collections (user_id, to_product, img, created_at) VALUES ($1,$2,$3,now())",
        &[&user_id, &to_product, &vec_img]
    ).await;
    let r = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    Ok(r)
}


