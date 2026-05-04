use chrono::{Utc};

use crate::{
    common::{PgPool},
    comments::models::{Cmt, Comment, VecCmt, FormComment},
};

impl FormComment {
    pub async fn insert_cmt(
        self,
        pool:    PgPool,
        user_id: i32,
        email:   String,
        name:    String,
        tab:     &str,
    ) -> Result<u64, Option<String>> {
        let v: Vec<Cmt> = vec![
            Cmt {
                user_id,
                tab_id:     self.to_id.unwrap_or_default(),
                email:      email.to_string(),
                name:       name.to_string(),
                msg:        self.comment.unwrap_or_default().to_string(),
                completed:  true,
                created_at: Utc::now(),
                updated_at: Some(Utc::now())
            }
        ];
        let cmt = serde_json::to_value(&v).unwrap();

        let mut s = String::from(
            "UPDATE  SET comments=ARRAY_APPEND(comments, $1) WHERE id=$2"
        );
        s.insert_str(7, tab);

        let pg = match pool.get().await{
            Ok(expr) => expr,
            Err(err) => return Err(Some(err.to_string()))
        };
        let result = pg.execute(&s, &[&cmt, &self.to_id]
        ).await;
        let r = match result {
            Ok(expr) => expr,
            Err(err) => return Err(Some(err.to_string()))
        };
        Ok(r)
    }
}


pub async fn insert_comment(
    pool: PgPool,
    user_id: i32,
    email:   String,
    name:    String,
    f:       FormComment,
    tab:     &str,
) -> Result<u64, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let v: Vec<Cmt> = vec![
        Cmt {
            user_id,
            tab_id:     f.to_id.unwrap_or_default(),
            email:      email.to_string(),
            name:       name.to_string(),
            msg:        f.comment.unwrap_or_default().to_string(),
            completed:  true,
            created_at: Utc::now(),
            updated_at: Some(Utc::now())
        }
    ];
    let cmt = serde_json::to_value(v).unwrap();

    let mut s = String::from(
        "UPDATE  SET comments=ARRAY_APPEND(comments, $1) WHERE id=$2"
    );
    s.insert_str(7, tab);
    let result = pg.execute(&s, &[&cmt, &f.to_id]
    ).await;
    let r = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    Ok(r)
}


pub async fn list_cmt(
    pool:   PgPool,
    number: i32,
    tab:    &str,
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
    name: &str
) -> Result<Option<VecCmt>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query_one(
        "SELECT comments FROM users WHERE username=$1;", &[&name]
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