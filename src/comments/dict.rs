use chrono::{Utc};
use serde_json::{Value, json};

use crate::{
    common::{PgPool, IntoKVIter},
    comments::models::{JsCmt, JsonComment, FormJson, FormUpdateCmt}
};

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
        "SELECT ARRAY_UPPER(ARRAY(SELECT JSONB_OBJECT_KEYS(dict)), 1) FROM  WHERE id=$1;"
    );
    x.insert_str(66, tab);
    let r_x = pg.query_one(&x, &[&to_id]).await;
    let row = match r_x {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let index = row.try_get::<_, i32>(0).unwrap_or(0);

    let cmt: Value = json!({
        index.to_string(): JsCmt {
            id:         index,
            to_id,
            user_id,
            tab_id:     to_id,
            email:      email.to_string(),
            name:       name.to_string(),
            msg:        f.msg,
            completed:  false,
            created_at: Utc::now(),
            updated_at: None
        }
    });

    let mut b = String::from(
        "UPDATE  SET dict=dict || $1 WHERE id=$2"
    );
    b.insert_str(7, tab);
    let result = pg.execute(&b, &[&cmt, &to_id]).await;
    let r = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    Ok(r)
}

pub async fn all_cmt(
    pool:  PgPool,
    to_id: i32,
    tab:   &str
) -> Result<Option<Value>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut s = String::from(
        "SELECT dict FROM  WHERE id=$1;"
    );
    s.insert_str(17, tab);
    let result = pg.query_one(&s, &[&to_id]
    ).await;
    let row = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };

    let r: Value = row.get("dict");
    r.items().unwrap();
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
        r#"SELECT jsonb_path_query(dict,format('$."%s" ? (@.user_id == %s && @.id == %s)', $1::text, $2::int, $3::int)::jsonpath) FROM  WHERE id=$4;"#
    );
    x.insert_str(124, tab);
    let row = pg.query_one(&x, &[&cid.to_string(), &user_id, &cid, &to_id]
    ).await.unwrap();
    // let row = match result {
    //     Ok(expr) => expr,
    //     Err(err) => return Err(Some(err.to_string()))
    // };
    let mut r = JsCmt::default();
    if !row.is_empty() {
        let cmt: JsonComment = JsonComment{comments: row.get(0)};
        let str_msg: String = serde_json::to_string(&cmt.comments).unwrap();
        r = serde_json::from_str::<JsCmt>(&str_msg).unwrap();
    }
    Ok(Some(r))
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
        "UPDATE  SET dict=JSONB_SET(dict, ARRAY[$1::text,'msg'], to_jsonb($2::text), false) WHERE id=$3 AND dict[$4::int]['user_id'] @> to_jsonb($5::int);"
    );
    x.insert_str(7, tab);
    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let r = pg.execute(
        &x, &[&cid.to_string(), &f.msg, &to_id, &cid, &user_id]
    ).await.unwrap();
    // let r = match result {
    //     Ok(expr) => expr,
    //     Err(err) => return Err(Some(err.to_string()))
    // };
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
        "UPDATE  SET dict=JSONB_SET(dict, ARRAY[$1::text,'msg'], to_jsonb($2::text), false) WHERE id=$3 AND dict[$4::int]['user_id'] @> to_jsonb($5::int);"
    );
    x.insert_str(7, tab);
    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };

    let pat = "deleted comment";
    let r = pg.execute(
        &x, &[&cid.to_string(), &pat, &to_id, &cid, &user_id]
    ).await.unwrap();
    // let r = match result {
    //     Ok(expr) => expr,
    //     Err(err) => return Err(Some(err.to_string()))
    // };
    Ok(r)
}


        // r#"UPDATE  SET dict=JSONB_SET(dict, ARRAY[$1::text,'msg'::text], to_jsonb($2::text), false) WHERE id=$3 AND dict @? format('$."%s" ? (@.id == %s && @.user_id == %s)', $4::text,$5::int,$6::int)::jsonpath;"#