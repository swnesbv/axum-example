use crate::{
    common::{PgPool},
};

pub async fn dialogue_joined(
    pool: PgPool,
    user_id: i32, joined: Option<String>, room: String
) -> bool {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(_) => return false
    };
    let result =
        pg.execute(
            "INSERT INTO chat_room (user_id, joined, room, created_at) VALUES ($1,$2,$3,now())",
            &[&user_id, &joined, &room]
        )
        .await;
    match result {
        Err(e) => {
            println!(" Err..! INSERT joined..!");
            println!(" err joined: [{}].\n", e);
            return false;
        }
        Ok(expr) => expr,
    };
    true
}

pub async fn insert_msg_room(
    pool: PgPool,
    user_id: i32, message: Option<String>, room: String
) -> bool {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(_) => return false
    };
    let result =
        pg.execute(
            "INSERT INTO chat_room (user_id, message, room, created_at) VALUES ($1,$2,$3,now())",
            &[&user_id, &message, &room]
        )
        .await;
    match result {
        Err(err) => {
            println!(" Err..! INSERT message");
            println!(" err msg: [{}].\n", err);
            return false;
        }
        Ok(expr) => expr,
    };
    true
}

pub async fn dialogue_came_out(
    pool: PgPool,
    user_id: i32, came_out: Option<String>, room: String
) -> bool {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(_) => return false
    };
    let result =
        pg.execute(
            "INSERT INTO chat_room (user_id, came_out, room, created_at) VALUES ($1,$2,$3,now())",
            &[&user_id, &came_out, &room]
        )
        .await;
    match result {
        Err(e) => {
            println!("Err..! came out INSERT");
            println!("err out: [{}].\n", e);
            return false;
        }
        Ok(o) => o,
    };
    true
}


pub async fn insert_joined(
    pool: PgPool,
    user_id: i32, joined: Option<String>
) -> bool {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(_) => return false
    };
    let result =
        pg.execute(
            "INSERT INTO chat_public (user_id, joined, created_at) VALUES ($1,$2,now())",
            &[&user_id, &joined]
        )
        .await;
    match result {
        Err(e) => {
            println!("Err..! INSERT joined");
            println!("Error message: [{}].\n", e);
            return false;
        }
        Ok(o) => o,
    };
    true
}

pub async fn insert_msg_pch(
     pool: PgPool,
    user_id: i32, message: Option<String>
) -> bool {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(_) => return false
    };
    let result =
        pg.execute(
            "INSERT INTO chat_public (user_id, message, created_at) VALUES ($1,$2,now())",
            &[&user_id, &message]
        )
        .await;
    match result {
        Err(e) => {
            println!("Err..! INSERT message");
            println!("Error message: [{}].\n", e);
            return false;
        }
        Ok(o) => o,
    };
    true
}

pub async fn insert_came_out(
    pool: PgPool,
    user_id: i32, came_out: Option<String>
) -> bool {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(_) => return false
    };
    let result =
        pg.execute(
            "INSERT INTO chat_public (user_id, came_out, created_at) VALUES ($1,$2,now())",
            &[&user_id, &came_out]
        )
        .await;
    match result {
        Err(e) => {
            println!("Err..! came out INSERT");
            println!("Error message: [{}].\n", e);
            return false;
        }
        Ok(o) => o,
    };
    true
}