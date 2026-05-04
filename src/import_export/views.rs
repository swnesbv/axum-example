use csv::Writer;
use std::error::Error;

use crate::{
    common::{PgPool},
    auth::models::AllUser
};

pub async fn all(
    pool: PgPool,
) -> Result<Vec<AllUser>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query(
        "SELECT id,email,username,password,img,status,created_at,updated_at FROM users;", &[]
    ).await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };

    let mut r: Vec<AllUser> = vec![];
    for i in rows {
        r.push(AllUser {
            id:         i.get(0),
            email:      i.get(1),
            username:   i.get(2),
            password:   i.get(3),
            img:        i.get(4),
            status:     i.get(5),
            created_at: i.get(6),
            updated_at: i.get(7)
        })
    }
    Ok(r)
}


pub fn rem_last(v: &str) -> String {
    let mut chars = v.chars();
    chars.next_back();
    chars.as_str().to_string()
}
pub fn w_status(
    v: Vec<String>
) -> String {
    let mut s = String::from("");
    for i in v {
        s.push_str(&i);
        s.push(',');
    }
    rem_last(&s)
}


pub async fn write_to_csv(data: Vec<AllUser>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_writer(vec![]);
    for pat in data {
        wtr.serialize(pat)?;
    }
    wtr.flush()?;
    Ok(())
}
