use crate::{
    common::{PgPool},
    purchases::models::{Purchases}
};

pub async fn all_purchases(
    pool: PgPool, id: i32
) -> Result<Vec<Purchases>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query(
        "SELECT * FROM purchases WHERE id=$1", &[&id]
    ).await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut r: Vec<Purchases> = vec![];
    for i in rows {
        r.push(Purchases {
            id:         i.get(0),
            user_id:    i.get(1),
            product_id: i.get(2),
            categories: i.get(3),
            amount:     i.get(4),
            price:      i.get(5),
            completed:  i.get(6),
            created_at: i.get(7),
            updated_at: i.get(8)
        })
    }
    Ok(r)
}

pub async fn id_purchases(
    pool: PgPool, id: i32
) -> Result<Purchases, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result =  pg.query_one(
        "SELECT * FROM purchases WHERE id=$1", &[&id]
    ).await;
    let i = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let r: Purchases = Purchases {
        id:         i.get(0),
        user_id:    i.get(1),
        product_id: i.get(2),
        categories: i.get(3),
        amount:     i.get(4),
        price:      i.get(5),
        completed:  i.get(6),
        created_at: i.get(7),
        updated_at: i.get(8)
    };
    Ok(r)
}


// pub async fn form_on_off(
//     form: FormSelect,
// ) -> Vec<String> {

//     let a = form.on_off;
//     let b = form.categories;

//     let mut f: Vec<String> = vec![];
//     let mut e = vec![];

//     for i in a {
//         let g = i.parse::<String>().unwrap();
//         f.push(g);
//     }
//     for (c, d) in f.iter().zip(b.iter()) {
//         if *c == "1" {
//             e.push(d.to_owned());
//         }
//     }
//     e
// }


// pub async fn i_categories(
//     pool: PgPool, a: Option<&str>, b: Option<&str>, c: Option<&str>, d: Option<&str>
// ) -> Result<Vec<Products>, String> {

//     let result = sqlx::query_as!(
//             Products,
//             "SELECT * FROM products WHERE categories && ARRAY[$1,$2,$3,$4]::TEXT[]", a,b,c,d
//         )
//         .fetch_all(&pool)
//         .await
//         .unwrap();
//     Ok(result)
// }
