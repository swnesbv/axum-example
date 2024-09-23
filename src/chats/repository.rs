use sqlx::PgConnection;
use crate::{
	chats::models::{PublicChat, Room, DialogueChat},
	subscriptions::models::{ToDialogue},
};


pub async fn all_public(
	conn: &mut PgConnection
) -> Result<Vec<PublicChat>, String> {

	let result = sqlx::query_as!(PublicChat, "SELECT * FROM chat_public")
		.fetch_all(&mut *conn)
		.await
		.unwrap();
	Ok(result)
}


pub async fn ssc_dialogue(
	conn: &mut PgConnection,
	to_user: i32,
) -> Result<Vec<ToDialogue>, String> {

	let result = sqlx::query_as!(
		ToDialogue,
		"SELECT to_user, additionally, completed FROM subscriptions WHERE to_user=$1 AND completed=$2",
		to_user, true
	)
	.fetch_all(&mut *conn)
	.await
	.unwrap();
	Ok(result)
}


pub async fn to_room(
	conn: &mut PgConnection,
	room: String,
) -> Result<Vec<Room>, String> {

	let result = sqlx::query_as!(
		Room,
		"SELECT * FROM chat_room WHERE room=$1",
		room
	)
	.fetch_all(&mut *conn)
	.await
	.unwrap();
	Ok(result)
}


pub async fn to_dialogue(
	conn: &mut PgConnection,
	dialogue: String,
) -> DialogueChat {

	sqlx::query_as!(
		DialogueChat,
		"SELECT user_id, to_user FROM subscriptions WHERE dialogue=$1",
		dialogue
	)
	.fetch_one(&mut *conn)
	.await
	.unwrap()
}


pub async fn total_dialogue(
	conn: &mut PgConnection,
	user_id: i32,
) -> i64 {

	let result = sqlx::query_scalar("SELECT COUNT(*) FROM chat_room WHERE user_id=$1")
		.bind(user_id)
		.fetch_one(&mut *conn)
		.await
		.unwrap();
	result
}

pub async fn user_id_dialogue(
	conn: &mut PgConnection, user_id: i32, limit: i64, offset: i64
) -> Result<Vec<Room>, String> {

	let result = sqlx::query_as!(
		Room,
		"SELECT * FROM chat_room WHERE user_id=$1 ORDER BY id LIMIT $2 OFFSET $3",
		user_id, limit, offset
	)
	.fetch_all(&mut *conn)
	.await
	.unwrap();
	Ok(result)
}

pub async fn vec_del_dialogue(
	conn: &mut PgConnection, id: Vec<i32>, user_id: i32
) -> bool {

	for i in id {
		let result = sqlx::query(
			"DELETE FROM chat_room WHERE id=$1 AND user_id=$2"
		)
		.bind(i)
		.bind(user_id)
		.execute(&mut *conn)
		.await;

		match result {
			Err(e) => {
				println!("Err DELETE: {}\n", e);
				return false;
			}
			Ok(res) => {
				println!("DELETE number: {} has been deleted.", i);
				println!("Amount deleted: {}", res.rows_affected());
			}
		}
	}
	true
}

pub async fn del_dialogue(
	conn: &mut PgConnection, id: i32, user_id: i32
) -> bool {

	let result = sqlx::query(
		"DELETE FROM chat_room WHERE id=$1 AND user_id=$2"
	)
	.bind(id)
	.bind(user_id)
	.execute(&mut *conn)
	.await;

	match result {
		Err(e) => {
			println!("Err DELETE: {}\n", e);
			return false;
		}
		Ok(res) => {
			println!("DELETE number: {} has been deleted.", id);
			println!("Amount deleted: {}", res.rows_affected());
		}
	}
	true
}
