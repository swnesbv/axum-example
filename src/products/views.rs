use crate::{
    common::PgPool,
    products::models::{Products, FormSelect}
};


pub async fn all_products(
    pool: PgPool
) -> Result<Vec<Products>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query("SELECT * FROM products;", &[]).await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut r: Vec<Products> = vec![];
    for i in rows {
        r.push(Products {
            id:           i.get(0),
            user_id:      i.get(1),
            title:        i.get(2),
            description:  i.get(3),
            categories:   i.get(4),
            cts:          i.get(5),
            amount:       i.get(6),
            price:        i.get(7),
            img:          i.get(8),
            completed:    i.get(9),
            created_at:   i.get(10),
            updated_at:   i.get(11)
        })
    }
    Ok(r)
}

pub async fn id_products(
    pool: PgPool,
    id: i32
) -> Result<Products, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result =  pg.query_one(
        "SELECT * FROM products WHERE id=$1;", &[&id]
    ).await;
    let i = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let r: Products = Products {
        id:           i.get(0),
        user_id:      i.get(1),
        title:        i.get(2),
        description:  i.get(3),
        categories:   i.get(4),
        cts:          i.get(5),
        amount:       i.get(6),
        price:        i.get(7),
        img:          i.get(8),
        completed:    i.get(9),
        created_at:   i.get(10),
        updated_at:   i.get(11)
    };
    Ok(r)
}


pub async fn form_on_off(
    f: FormSelect,
) -> Vec<String> {

    let a = f.on_off;
    let b = f.categories;

    let mut v: Vec<String> = vec![];
    let mut e = vec![];

    for x in a {
        let y = x.parse::<String>().unwrap();
        v.push(y);
    }
    for (c, d) in v.iter().zip(b.iter()) {
        if *c == "1" {
            e.push(d.to_owned());
        }
    }
    e
}


pub async fn i_categories(
    pool: PgPool,
    a: Option<&str>, b: Option<&str>, c: Option<&str>, d: Option<&str>
) -> Result<Vec<Products>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query(
        "SELECT * FROM products WHERE categories && ARRAY[$1,$2,$3,$4]::TEXT[]",
        &[&a, &b, &c, &d]
    ).await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut r: Vec<Products> = vec![];
    for i in rows {
        r.push(Products {
            id:           i.get(0),
            user_id:      i.get(1),
            title:        i.get(2),
            description:  i.get(3),
            categories:   i.get(4),
            cts:          i.get(5),
            amount:       i.get(6),
            price:        i.get(7),
            img:          i.get(8),
            completed:    i.get(9),
            created_at:   i.get(10),
            updated_at:   i.get(11)
        })
    }
    Ok(r)
}

pub async fn i_cts(
    pool: PgPool,
    i: Option<Vec<String>>
) -> Result<Vec<Products>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result =  pg.query(
        "SELECT * FROM products WHERE $1 @> cts;", &[&i.as_deref()]
    ).await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut r: Vec<Products> = vec![];
    for i in rows {
        r.push(Products {
            id:           i.get(0),
            user_id:      i.get(1),
            title:        i.get(2),
            description:  i.get(3),
            categories:   i.get(4),
            cts:          i.get(5),
            amount:       i.get(6),
            price:        i.get(7),
            img:          i.get(8),
            completed:    i.get(9),
            created_at:   i.get(10),
            updated_at:   i.get(11)
        })
    }
    Ok(r)
}
