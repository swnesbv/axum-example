use crate::{
    common::PgPool,
    products::models::{ProductsSlider}
};


pub async fn slider_products(
    pool: PgPool,
    to_product: i32
) -> Result<ProductsSlider, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query_one(
        "SELECT * FROM slider WHERE to_product=$1;", &[&to_product]
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

