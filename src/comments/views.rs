use chrono::{Utc};
use rand::distr::{Alphanumeric, SampleString};

use crate::{
    common::{PgPool},
    comments::models::{Comment, Cmt, VecCmt, FormComment}
};

pub async fn id_cmt(
    pool: PgPool,
    name: &str,
    cid:  &str,
    tab:  &str,
) -> Result<Cmt, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };

    let mut s = String::from(
        "SELECT comments FROM  WHERE username=$1;"
    );
    s.insert_str(21, tab);
    let result = pg.query_one(&s, &[&name]
    ).await;
    let row = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };

    let mut index = -1;
    let mut b = VecCmt::default();
    let v: Comment = Comment{comments: row.get("comments")};
    if v.comments.is_some() {
        let a: String = serde_json::to_string(&v).unwrap();
        b = serde_json::from_str::<VecCmt>(&a).unwrap();
        for i in &b.comments {
            index += 1;
            if i.id == cid {
                break
            }
        }
    }
    let r: Cmt = b.comments[index as usize].clone();
    println!(" id_cmt index..! {:?}", index);
    Ok(r)
}

pub async fn arrayposition(
    pool: PgPool,
    name: &str,
    cid:  &str,
    tab:  &str,
) -> Result<usize, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };

    let mut s = String::from(
        "SELECT comments FROM  WHERE username=$1;"
    );
    s.insert_str(21, tab);
    let result = pg.query_one(&s, &[&name]
    ).await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };

    let mut r = 0;
    let v: Comment = Comment{comments: rows.get("comments")};
    if v.comments.is_some() {
        let a: String   = serde_json::to_string(&v).unwrap();
        let b = serde_json::from_str::<VecCmt>(&a).unwrap();
        for i in &b.comments {
            r += 1;
            if i.id == cid {
                break
            }
        }
    }
    println!(" arrayposition usize..! {:?}", r as usize);
    println!(" arrayposition..! {:?}", r);
    Ok(r as usize)
}

pub async fn len_cmt(
    pool: PgPool,
    name: &str,
    tab:  &str,
) -> Result<i32, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut s = String::from(
        "SELECT comments FROM  WHERE username=$1;"
    );
    s.insert_str(21, tab);
    let result = pg.query_one(&s, &[&name]
    ).await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };

    let mut r = VecCmt::default();
    let v: Comment = Comment{comments: rows.get("comments")};
    if v.comments.is_some() {
        let str_msg = serde_json::to_string(&v).unwrap();
        r = serde_json::from_str::<VecCmt>(&str_msg).unwrap();
    }
    Ok(r.comments.len() as i32)
}

pub async fn insert_comment(
    pool:  PgPool,
    user:  i32,
    email: String,
    name:  String,
    f:     FormComment,
    tab:   &str,
) -> Result<u64, Option<String>> {

    let dialogue = Alphanumeric.sample_string(
        &mut rand::rng(), 4
    );
    let v: Cmt = Cmt {
        id:        dialogue,
        user_id:   user,
        tab_id:    f.to_id.unwrap_or_default(),
        email:     email.to_string(),
        name:      name.to_string(),
        msg:       f.comment.unwrap_or_default().to_string(),
        completed: false,
        created_at: Utc::now(),
        updated_at: None
    };
    let cmt = serde_json::to_value(v).unwrap();
    let mut b = String::from(
        "UPDATE  SET comments=ARRAY_APPEND(comments, $1) WHERE id=$2"
    );
    b.insert_str(7, tab);
    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.execute(&b, &[&cmt, &f.to_id]).await;
    let r = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    Ok(r)
}

pub async fn list_cmt(
    pool:   PgPool,
    number: i32,
    tab:    &str
) -> Result<Option<VecCmt>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut s = String::from(
        "SELECT comments FROM  WHERE id=$1;"
    );
    s.insert_str(21, tab);
    let result = pg.query_one(&s, &[&number]
    ).await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };

    let mut r = VecCmt::default();
    let v: Comment = Comment{comments: rows.get("comments")};
    if v.comments.is_some() {
        let str_msg = serde_json::to_string(&v).unwrap();
        r = serde_json::from_str::<VecCmt>(&str_msg).unwrap();
        r.comments.sort_by(|a, b| b.cmp(a));
    }
    Ok(Some(r))
}

pub async fn i_comments(
    pool: PgPool,
    name: &str,
    tab:  &str,
) -> Result<Option<VecCmt>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut s = String::from(
        "SELECT comments FROM  WHERE username=$1;"
    );
    s.insert_str(21, tab);
    let result = pg.query_one(&s, &[&name]
    ).await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };

    let mut r = VecCmt::default();
    let v: Comment = Comment{comments: rows.get("comments")};
    if v.comments.is_some() {
        let str_msg = serde_json::to_string(&v).unwrap();
        r = serde_json::from_str::<VecCmt>(&str_msg).unwrap();
        r.comments.sort_by(|a, b| b.cmp(a));
    }
    Ok(Some(r))
}


impl FormComment {
    pub async fn insert_cmt(
        self,
        pool:    PgPool,
        user:    i32,
        email:   String,
        name:    String,
        tab:     &str,
    ) -> Result<u64, Option<String>> {

        let dialogue = Alphanumeric.sample_string(
            &mut rand::rng(), 6
        );
        let v: Cmt = Cmt {
            id:        dialogue,
            user_id:   user,
            tab_id:    self.to_id.unwrap_or_default(),
            email:     email.to_string(),
            name:      name.to_string(),
            msg:       self.comment.unwrap_or_default().to_string(),
            completed: false,
            created_at: Utc::now(),
            updated_at: None
        };
        let cmt = serde_json::to_value(&v).unwrap();

        let mut s = String::from(
            "UPDATE  SET comments=ARRAY_APPEND(comments, $1) WHERE id=$2"
        );
        s.insert_str(7, tab);
        let pg = match pool.get().await{
            Ok(expr) => expr,
            Err(err) => return Err(Some(err.to_string()))
        };
        let b = pg.execute(&s, &[&cmt, &self.to_id]
        ).await;
        let r = match b {
            Ok(expr) => expr,
            Err(err) => return Err(Some(err.to_string()))
        };
        Ok(r)
    }
}