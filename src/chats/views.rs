use chrono::Utc;
use sqlx::PgConnection;


pub async fn dialogue_joined(
    conn: &mut PgConnection,
    user_id: i32, joined: Option<String>, room: String
) -> bool {

    let result =
        sqlx::query("INSERT INTO chat_room (user_id, joined, room, created_at) VALUES ($1,$2,$3,$4)")
            .bind(user_id)
            .bind(joined)
            .bind(room)
            .bind(Utc::now())
            .execute(&mut *conn)
            .await;

    match result {
        Err(e) => {
            println!("Err..! INSERT joined");
            println!("err joined: [{}].\n", e);
            return false;
        }
        Ok(o) => o,
    };
    true
}

pub async fn dialogue_message(
    conn: &mut PgConnection,
    user_id: i32, message: Option<String>, room: String
) -> bool {

    let result =
        sqlx::query("INSERT INTO chat_room (user_id, message, room, created_at) VALUES ($1,$2,$3,$4)")
            .bind(user_id)
            .bind(message)
            .bind(room)
            .bind(Utc::now())
            .execute(&mut *conn)
            .await;

    match result {
        Err(e) => {
            println!("Err..! INSERT message");
            println!("err msg: [{}].\n", e);
            return false;
        }
        Ok(o) => o,
    };
    true
}

pub async fn dialogue_came_out(
    conn: &mut PgConnection,
    user_id: i32, came_out: Option<String>, room: String
) -> bool {

    let result =
        sqlx::query("INSERT INTO chat_room (user_id, came_out, room, created_at) VALUES ($1,$2,$3,$4)")
            .bind(user_id)
            .bind(came_out)
            .bind(room)
            .bind(Utc::now())
            .execute(&mut *conn)
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
    conn: &mut PgConnection, user_id: i32, joined: Option<String>
) -> bool {

    let result =
        sqlx::query(
            "INSERT INTO chat_public (user_id, joined, created_at) VALUES ($1,$2,$3)"
        )
        .bind(user_id)
        .bind(joined)
        .bind(Utc::now())
        .execute(&mut *conn)
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

pub async fn insert_message(
    conn: &mut PgConnection, user_id: i32, message: Option<String>
) -> bool {

    let result =
        sqlx::query("INSERT INTO chat_public (user_id, message, created_at) VALUES ($1,$2,$3)")
            .bind(user_id)
            .bind(message)
            .bind(Utc::now())
            .execute(&mut *conn)
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
    conn: &mut PgConnection, user_id: i32, came_out: Option<String>
) -> bool {

    let result =
        sqlx::query("INSERT INTO chat_public (user_id, came_out, created_at) VALUES ($1,$2,$3)")
            .bind(user_id)
            .bind(came_out)
            .bind(Utc::now())
            .execute(&mut *conn)
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