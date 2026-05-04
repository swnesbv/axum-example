use crate::{
    common::{PgPool},
    booking::models::{ListBkg},
};


pub async fn all(
    pool: PgPool
) -> Result<Vec<ListBkg>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query("SELECT * FROM booking;", &[]).await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut r: Vec<ListBkg> = vec![];
    for i in rows {
        r.push(ListBkg {
            id:             i.get("id"),
            user_id:        i.get("user_id"),
            provision_d_id: i.get("provision_d_id"),
            provision_h_id: i.get("provision_h_id"),
            title:          i.get("title"),
            description:    i.get("description"),
            st_date:        i.get("st_date"),
            en_date:        i.get("en_date"),
            st_hour:        i.get("st_hour"),
            en_hour:        i.get("en_hour"),
            completed:      i.get("completed"),
            created_at:     i.get("created_at"),
            updated_at:     i.get("updated_at")
        })
    }
    Ok(r)
}
