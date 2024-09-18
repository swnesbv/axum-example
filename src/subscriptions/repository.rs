use sqlx::postgres::PgPool;

use crate::subscriptions::models::{Group, Subscription};


pub async fn ssc_owner(
	pool: PgPool,
	user_id: i32,
) -> Result<Vec<Subscription>, String> {

	let result = sqlx::query_as!(
		Subscription,
		"SELECT * FROM subscriptions WHERE user_id=$1",
		user_id
	)
	.fetch_all(&pool)
	.await
	.unwrap();
	Ok(result)
}

pub async fn ssc_to_user(
	pool: PgPool,
	user_id: i32,
) -> Result<Vec<Subscription>, String> {

	let result = sqlx::query_as!(
		Subscription,
		"SELECT * FROM subscriptions WHERE to_user=$1",
		user_id
	)
	.fetch_all(&pool)
	.await
	.unwrap();
	Ok(result)
}

pub async fn to_ssc_user(
	pool: PgPool,
	to_user: i32,
) -> Result<Vec<Subscription>, String> {

	let result = sqlx::query_as!(
		Subscription,
		"SELECT * FROM subscriptions WHERE to_user=$1 AND completed=$2",
		to_user, false
	)
	.fetch_all(&pool)
	.await
	.unwrap();
	Ok(result)
}


pub async fn all_groups(
	pool: PgPool
) -> Result<Vec<Group>, String> {

	let result = sqlx::query_as!(Group, "SELECT * FROM groups")
		.fetch_all(&pool)
		.await
		.unwrap();
	Ok(result)
}

pub async fn all_groups_user(
	pool: PgPool,
	user_id: i32,
) -> Result<Vec<Group>, String> {

	let result = sqlx::query_as!(
		Group,
		"SELECT * FROM groups WHERE user_id=$1",
		user_id
	)
	.fetch_all(&pool)
	.await
	.unwrap();
	Ok(result)
}

pub async fn to_ssc_group(
	pool: PgPool,
	to_group: i32,
) -> Result<Vec<Subscription>, String> {

	let result = sqlx::query_as!(
		Subscription,
		"SELECT * FROM subscriptions WHERE to_group=$1 AND completed=$2",
		to_group, false
	)
	.fetch_all(&pool)
	.await
	.unwrap();
	Ok(result)
}