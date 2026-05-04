use crate::{
    common::{PgPool},
    profile::models::{ListUser, UpdateUser},
};

pub async fn all(
    pool: PgPool,
) -> Result<Vec<ListUser>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };

    let result = pg.query(
        "SELECT id,email,username,img,status,created_at,updated_at FROM users;", &[]
    ).await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut r: Vec<ListUser> = vec![];
    for i in rows {
        r.push(ListUser {
            id:         i.get("id"),
            email:      i.get("email"),
            username:   i.get("username"),
            img:        i.get("img"),
            status:     i.get("status"),
            created_at: i.get("created_at"),
            updated_at: i.get("updated_at")
        })
    }
    Ok(r)
}

pub async fn update_details(
    pool: PgPool, id: i32
) -> Result<UpdateUser, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query_one("SELECT email,username,updated_at FROM users WHERE id=$1;", &[&id])
    .await;
    let i = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let r = UpdateUser {email: i.get(0), username: i.get(1), updated_at: Some(i.get(2))};
    Ok(r)
}

pub async fn details(
    pool: PgPool,
    name: String
) -> Result<ListUser, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query_one(
        "SELECT id,email,username,img,status,created_at,updated_at FROM users WHERE username=$1",
        &[&name]
    )
    .await;
    let i = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let r = ListUser {
        id:         i.get("id"),
        email:      i.get("email"),
        username:   i.get("username"),
        img:        i.get("img"),
        status:     i.get("status"),
        created_at: i.get("created_at"),
        updated_at: i.get("updated_at")
    };
    Ok(r)
}

pub async fn del_user(
    pool: PgPool,
    id: i32, email: String
) -> Result<u64, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.execute(
        "DELETE FROM users WHERE id=$1 AND email=$2",
        &[&id, &email]
    )
    .await;
    match result {
        Ok(expr) => Ok(expr),
        Err(err) => Err(Some(err.to_string()))
    }
}

pub async fn del_admin(
    pool: PgPool,
    id: i32
) -> Result<u64, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.execute(
        "DELETE FROM users WHERE id=$1",
        &[&id]
    )
    .await;
    match result {
        Ok(expr) => Ok(expr),
        Err(err) => Err(Some(err.to_string()))
    }
}