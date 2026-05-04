use crate::{
    common::{PgPool},
    auth::models::{AuToken},
    subscriptions::models::{Key}
};

pub async fn insert_ssc_user(
    pool: PgPool,
    user_id: i32,
    title: &str,
    description: &str,
    to_user: i32,
    user: AuToken,
) -> Result<u64, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };

    let key: Key = Key {
        email: user.email,
        name: user.username,
    };
    let s = serde_json::to_string(&key).unwrap();
    let v: serde_json::Value = serde_json::from_str(&s).unwrap();
    let result = pg.execute(
        "INSERT INTO subscriptions (user_id, title, description, to_user, additionally, created_at) VALUES ($1,$2,$3,$4,$5,now())",
        &[&user_id, &title, &description, &to_user, &v]
    ).await;
    let r = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    Ok(r)
}

pub async fn insert_ssc_group(
    pool: PgPool,
    user_id: i32,
    title: &str,
    description: &str,
    to_group: i32,
    user: AuToken
) -> Result<u64 , Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };

    let key = Key {
        email: user.clone().email,
        name: user.clone().username,
    };
    let s = serde_json::to_string(&key).unwrap();
    println!("insert_ssc_group s.. {:#?}", s);

    let dialogue = vec![user_id];
    let result =
        pg.execute(
            "INSERT INTO subscriptions (user_id, title, description, to_group, dialogue, additionally, created_at) VALUES ($1,$2,$3,$4,$5,$6,now())",
            &[&user_id, &title, &description, &to_group, &dialogue, &s.to_string()]
        )
        .await;
    let r = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    Ok(r)
}

pub async fn resolution_user(
    pool: PgPool,
    id: i32,
    dialogue: String,
) -> Result<u64 , Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result =
        pg.execute(
            "UPDATE subscriptions SET dialogue=$2, completed=$3, updated_at=now() WHERE id=$1",
            &[&id, &dialogue, &true]
        )
        .await;
    let r = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    Ok(r)
}

pub async fn resolution_group(
    pool: PgPool,
    id: i32,
    to_group: i32,
) -> Result<u64 , Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.execute(
        "UPDATE subscriptions SET dialogue=ARRAY_APPEND(dialogue, $2), completed=$3, updated_at=now() WHERE id=$1",
        &[&id, &to_group, &true]
    )
    .await;
    let r = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    Ok(r)
}