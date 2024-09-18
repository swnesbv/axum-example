use crate::{
    comments::models::{Comments},
};


pub async fn i_comments(
    conn: &mut sqlx::PgConnection,
    i: &str
) -> Result<Vec<Comments>, String> {

    let result = sqlx::query_as!(
            Comments,
            "SELECT * FROM comments WHERE comment_on::JSON ->> 'whose'=$1", i
        )
        .fetch_all(&mut *conn)
        .await
        .unwrap();
    Ok(result)
}
