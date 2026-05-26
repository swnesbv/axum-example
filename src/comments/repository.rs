use std::mem;
use chrono::{Utc};

use crate::{
    common::{PgPool, to_bool},
    comments::views::{id_cmt, arrayposition},
    comments::models::{Cmt, FormUpdateCmt}
};

pub async fn cmt_del(
    pool: PgPool,
    name: &str,
    cid:  &str,
    tab:  &str,
) -> Result<u64, Option<String>> {

    let index = arrayposition(
        pool.clone(), name, cid, "users"
    ).await.unwrap();
    println!(" cmt_del index.. {:?}", index);

    let mut del = String::from(
        "UPDATE  SET comments=ARRAY_REMOVE(comments, comments[$1]) WHERE username=$2"
    );
    del.insert_str(7, tab);
    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.execute(&del, &[&(index as i32), &name]).await;
    let r = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    Ok(r)
}

pub async fn replace_cms(
    pool: PgPool,
    name: &str,
    cid:  &str,
    f:    FormUpdateCmt,
    tab:  &str,
) -> Result<u64, Option<String>> {

    let mut i = id_cmt(
        pool.clone(), name, cid, "users"
    ).await.unwrap();
    println!(" old.. {:#?}", i);

    let on_off = match f.completed {
        Some(expr) => expr,
        None => false.to_string()
    };
    let cpt = to_bool(&on_off);
    let a: Cmt = Cmt {
        id:         i.id.to_owned(),
        user_id:    i.user_id,
        tab_id:     i.tab_id,
        email:      i.email.clone(),
        name:       i.name.clone(),
        msg:        f.msg.unwrap_or_default().to_string(),
        completed:  cpt,
        created_at: i.created_at,
        updated_at: Some(Utc::now())
    };
    let b = serde_json::to_string(&a).unwrap();
    let cmt: Cmt = serde_json::from_str(&b).unwrap();

    let _ = mem::replace(&mut i, cmt.clone());
    let re_val = serde_json::to_value(a).unwrap();
    println!(" re_val.. {:#?}", re_val);

    let index = arrayposition(
        pool.clone(), name, cid, "users"
    ).await.unwrap();
    println!(" replace_cms index.. {:?}", index);

    let mut upd = String::from(
        "UPDATE  SET comments=ARRAY_REPLACE(comments, comments[$1], $2) WHERE username=$3"
    );
    upd.insert_str(7, tab);
    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.execute(&upd, &[&(index as i32), &re_val, &name]).await;
    let r = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    Ok(r)
}
