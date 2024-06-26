use axum::{
    extract::{State},
};

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{schema};
use crate::{
    common::{Pool},
    provision::models::{
        AllPrD,
    	BkgPrD,
        UpPrdBkg,
    },
};


pub async fn all_days(
    State(pool): State<Pool>,
) -> QueryResult<Vec<AllPrD>> {

    let mut conn = pool.get().await.unwrap();
    use schema::provision_d::dsl::*;
    let all = provision_d.select(AllPrD::as_select())
        .load(&mut conn)
        .await
        .unwrap();

    Ok(all)
}


pub async fn creat_bkg(
	State(pool): State<Pool>,
	bkg: BkgPrD, 
) -> QueryResult<BkgPrD> {

	let mut conn = pool.get().await.unwrap();
	use schema::booking::dsl::*;
    diesel::insert_into(booking)
        .values(&bkg)
        .returning(BkgPrD::as_returning())
        .get_result(&mut conn)
        .await
        .unwrap();

    Ok(bkg)
}

pub async fn update_prv(
    State(pool): State<Pool>,
    prv: UpPrdBkg, 
    number: i32,
) -> QueryResult<UpPrdBkg> {

    let mut conn = pool.get().await.unwrap();
    use schema::provision_d::dsl::*;
    diesel::update(provision_d.filter(id.eq(number)))
        .set(&prv)
        .execute(&mut conn)
        .await
        .unwrap();

    Ok(prv)
}