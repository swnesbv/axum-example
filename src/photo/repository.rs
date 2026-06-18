use axum::{
    body::Body,
    http::{Response, StatusCode},
};

use crate::{
    common::PgPool,
    products::models::{ProductsSlider},
    photo::models::{Img, VecImg, Collections}
};

pub async fn slider_id(
    pool: PgPool,
    id: i32,
    user_id: i32
) -> Result<ProductsSlider, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query_one(
        "SELECT * FROM slider WHERE id=$1 AND user_id=$2;", &[&id, &user_id]
    )
    .await;
    let i = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };

    let j_title: String = serde_json::to_string::<serde_json::Value>(
        &i.get::<&str, serde_json::Value>("title")
    ).unwrap();
    let a: Vec<String> = serde_json::from_str(&j_title).unwrap();

    let j_description: String = serde_json::to_string::<serde_json::Value>(
        &i.get::<&str, serde_json::Value>("description")
    ).unwrap();
    let b: Vec<String> = serde_json::from_str(&j_description).unwrap();

    let j_img: String = serde_json::to_string::<serde_json::Value>(
        &i.get::<&str, serde_json::Value>("img")
    ).unwrap();
    let c: Vec<String> = serde_json::from_str(&j_img).unwrap();

    let r: ProductsSlider = ProductsSlider {
        id:           i.get("id"),
        user_id:      i.get("user_id"),
        to_product:   i.get("to_product"),
        title:        a,
        description:  b,
        img:          c,
        completed:    i.get("completed"),
        created_at:   i.get("created_at"),
        updated_at:   i.get("updated_at")
    };
    Ok(r)
}

pub async fn zip_collection(
    pool: PgPool,
    user_id: i32
) -> Result<Vec<Collections>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query(
        "SELECT * FROM collections WHERE user_id=$1;", &[&user_id]
    ).await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut r: Vec<Collections> = vec![];
    rows.iter().for_each(|i| {
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
    });
    Ok(r)
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
        "SELECT img FROM slider WHERE id=$1;", &[&id]
    )
    .await;
    let row = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut r = VecImg::default();
    let v: Img = Img{img: row.get("img")};
    if v.img.is_some() {
        let str_msg = serde_json::to_string(&v).unwrap();
        r = serde_json::from_str::<VecImg>(&str_msg).unwrap();
        r.img.sort_by(|a, b| b.cmp(a));
    }
    Ok(Some(r))
}

pub fn del_msg(
    err:   String,
    alert: String,
    url:   String
) -> Response<Body> {

    let mut s = url.clone();
    if s.contains("?") {
        let s_offset = s.rfind('?').unwrap_or(s.len());
         s.replace_range(s_offset.., "");
    }
    let token = err + "," + &alert;
    Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", url)
        .header(
            "Set-Cookie",
            format!(
                "{}={}; Path={}; HttpOnly={}; SameSite={}; Max-Age={};",
                "to_msg", token, s, "true", "lax", 60
            ),
        )
        .body(Body::from("not found"))
        .unwrap()
}

pub async fn add_msg(
    err:   String,
    alert: String,
    url:   String
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