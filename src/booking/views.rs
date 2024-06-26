// use axum::{
//     extract::{State},
// };

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{schema};
use crate::{
    common::{DBConnection},
    booking::models::{
    	LtBkg,
        // UpPrdBkg,
    },
};


pub async fn all_bkg(
	DBConnection(mut conn): DBConnection,
) -> QueryResult<Vec<LtBkg>> {

	use schema::booking::dsl::*;

    let bkg = booking.select(LtBkg::as_select())
        .load(&mut conn)
        .await
        .unwrap();

    Ok(bkg)
}