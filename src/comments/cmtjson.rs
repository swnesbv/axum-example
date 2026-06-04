use chrono::{Utc};

use crate::{
    common::{PgPool},
    comments::models::{
        JsonComment, JsCmt, JsVecCmt, FormJson, FormUpdateCmt
    }
};

pub async fn all_cmt(
    pool:  PgPool,
    to_id: i32,
    tab:   &str
) -> Result<Option<JsVecCmt>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut s = String::from(
        "SELECT cmtjson FROM  WHERE id=$1;"
    );
    s.insert_str(20, tab);
    let result = pg.query_one(&s, &[&to_id]
    ).await;
    let row = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };

    let mut r = JsVecCmt::default();
    let v: JsonComment = JsonComment{comments: row.get("cmtjson")};
    if v.comments.is_some() {
        let str_msg = serde_json::to_string(&v).unwrap();
        r = serde_json::from_str::<JsVecCmt>(&str_msg).unwrap();
    }
    Ok(Some(r))
}

pub async fn id_cmt(
    pool:    PgPool,
    to_id:   i32,
    user_id: i32,
    cid:     i32,
    tab:     &str
) -> Result<Option<JsCmt>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };

    let mut x = String::from(
        "SELECT jsonb_path_query(cmtjson,format('$[*] ? (@.user_id == %s && @.id == %s)', $1::int, $2::int)::jsonpath) FROM  WHERE id=$3;"
    );
    x.insert_str(115, tab);
    let result = pg.query_one(&x, &[&user_id, &cid, &to_id]
    ).await;
    let row = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut r = JsCmt::default();
    if !row.is_empty() {
        let cmt: JsonComment = JsonComment{comments: row.get(0)};
        let str_msg: String = serde_json::to_string(&cmt.comments).unwrap();
        r = serde_json::from_str::<JsCmt>(&str_msg).unwrap();
    }
    Ok(Some(r))
}

pub async fn creat_cmt(
    pool:    PgPool,
    to_id:   i32,
    user_id: i32,
    email:   String,
    name:    String,
    f:       FormJson,
    tab:     &str,
) -> Result<u64, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut x = String::from(
        "SELECT JSONB_ARRAY_LENGTH(cmtjson) FROM  WHERE id=$1;"
    );
    x.insert_str(40, tab);
    let r_x = pg.query_one(&x, &[&to_id]
    ).await;
    let row = match r_x {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };

    let length: i32 = row.get::<_, i32>(0);
    let len: i32 = length + 1;

    let cmt: JsCmt = JsCmt {
        id:         len,
        to_id,
        user_id,
        tab_id:     to_id,
        email:      email.to_string(),
        name:       name.to_string(),
        msg:        f.msg,
        completed:  false,
        created_at: Utc::now(),
        updated_at: None
    };

    let val = serde_json::to_value(cmt).unwrap();
    let mut b = String::from(
        "UPDATE  SET cmtjson=JSONB_INSERT(cmtjson, array[$1::text], $2) WHERE id=$3"
    );
    b.insert_str(7, tab);
    let result = pg.execute(
        &b, &[&length.to_string(), &val, &to_id]
    ).await;
    let r = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    Ok(r)
}

pub async fn update_cmt(
    pool:    PgPool,
    to_id:   i32,
    user_id: i32,
    cid:     i32,
    f:       FormUpdateCmt,
    tab:     &str,
) -> Result<u64, Option<String>> {

    let mut x = String::from(
        "UPDATE  SET cmtjson=JSONB_SET(cmtjson, ARRAY[$1::text,'msg'], to_jsonb($2::text), false) WHERE id=$3 AND cmtjson[$4::int] @> $5::jsonb;"
    );
    x.insert_str(7, tab);
    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };

    let check: serde_json::Value = serde_json::json!(
        {"user_id": user_id}
    );
    let index = cid - 1;
    let result = pg.execute(
        &x, &[&index.to_string(), &f.msg, &to_id, &index, &check]
    ).await;
    let r = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    Ok(r)
}


pub async fn del_cmt(
    pool:    PgPool,
    cid:     i32,
    user_id: i32,
    to_id:   i32,
    tab:     &str,
) -> Result<u64, Option<String>> {

    let mut x = String::from(
        //"UPDATE  SET cmtjson=cmtjson #- '{0}' WHERE id=$2"
        "UPDATE  SET cmtjson=JSONB_SET(cmtjson, ARRAY[$1::text,'msg'], to_jsonb($2::text), false) WHERE id=$3 AND cmtjson[$4::int] @> $5::jsonb;"

    );
    x.insert_str(7, tab);
    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };

    let check: serde_json::Value = serde_json::json!(
        {"user_id": user_id}
    );
    let index = cid - 1;
    let pat = "deleted comment";
    let result = pg.execute(
        &x, &[&index.to_string(), &pat, &to_id, &index, &check]
    ).await;
    let r = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    Ok(r)
}