use axum::{
    body::Body,
    http::{Response, StatusCode},
};

use crate::{
    common::PgPool,
    photo::models::{Img, VecImg, Collections}
};

pub async fn zip_collection(
    pool: PgPool,
    user_id: i32
) -> Result<Vec<Collections>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query(
        "SELECT * FROM collections WHERE user_id=$1", &[&user_id]
    ).await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut r: Vec<Collections> = vec![];
    for i in rows {
        let a: String = serde_json::to_string::<serde_json::Value>(
            &i.get::<&str, serde_json::Value>("img")
        ).unwrap();
        let b: Vec<String> = serde_json::from_str(&a).unwrap();
        r.push(Collections {
            id:           i.get("id"),
            user_id:      i.get("user_id"),
            to_product:   i.get("to_product"),
            img:          b,
            completed:    i.get("completed"),
            created_at:   i.get("created_at"),
            updated_at:   i.get("updated_at")
        })
    }
    Ok(r)

    // let mut r = VecImg::default();
    // let v: Img = Img{img: rows.get("img")};
    // if v.img.is_some() {
    //     let str_msg = serde_json::to_string(&v).unwrap();
    //     r = serde_json::from_str::<VecImg>(&str_msg).unwrap();
    //     r.img.sort_by(|a, b| b.cmp(a));
    // }
    // Ok(Some(r))
}

pub async fn sl_photo(
    pool: PgPool,
    id: i32
) -> Result<Option<VecImg>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query_one(
        "SELECT img FROM slider WHERE id=$1",
        &[&id]
    )
    .await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut r = VecImg::default();
    let v: Img = Img{img: rows.get("img")};
    if v.img.is_some() {
        let str_msg = serde_json::to_string(&v).unwrap();
        r = serde_json::from_str::<VecImg>(&str_msg).unwrap();
        r.img.sort_by(|a, b| b.cmp(a));
    }
    Ok(Some(r))
}

pub async fn add_msg(
    err: String, alert: String, url: String
) -> Response<Body> {

    let token = err + "," + &alert;
    Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", &url)
        .header(
            "Set-Cookie",
            format!(
                "{}={}; Path={}; HttpOnly={}; SameSite={}; Max-Age={};",
                "to_msg", token, url, "true", "lax", 60
            ),
        )
        .body(Body::from("not found"))
        .unwrap()
}