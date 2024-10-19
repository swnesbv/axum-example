use sqlx::postgres::PgPool;

use crate::products::models::{Products, FormSelect};


pub async fn all_products(pool: PgPool) -> Result<Vec<Products>, String> {
    let result = sqlx::query_as!(Products, "SELECT * FROM products")
        .fetch_all(&pool)
        .await
        .unwrap();
    Ok(result)
}

pub async fn id_products(
    pool: PgPool, id: i32
) -> Result<Products, String> {

    let result = sqlx::query_as!(
        Products,
        "SELECT * FROM products WHERE id=$1",
        id
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    Ok(result)
}


pub async fn form_on_off(
    form: FormSelect,
) -> Vec<String> {

    let a = form.on_off;
    let b = form.categories;

    let mut f: Vec<String> = vec![];
    let mut e = vec![];

    for i in a {
        let g = i.parse::<String>().unwrap();
        f.push(g);
    }
    for (c, d) in f.iter().zip(b.iter()) {
        if *c == "1" {
            e.push(d.to_owned());
        }
    }
    e
}


pub async fn i_categories(
    pool: PgPool, a: Option<&str>, b: Option<&str>, c: Option<&str>, d: Option<&str>
) -> Result<Vec<Products>, String> {

    let result = sqlx::query_as!(
        Products,
        "SELECT * FROM products WHERE categories && ARRAY[$1,$2,$3,$4]::TEXT[]", a,b,c,d
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    Ok(result)
}
