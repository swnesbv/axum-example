use crate::{
	common::{PgPool},
	chats::models::{PublicChat, Room, DialogueChat},
	subscriptions::models::{ToDialogue},
};


pub async fn all_public(
	pool: PgPool,
) -> Result<Vec<PublicChat>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };

	let result = pg.query(
		"SELECT * FROM chat_public;", &[]
	)
	.await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let r: Vec<PublicChat> = rows.into_iter().map(
    	|i| PublicChat {
    		id: 	    i.get(0),
    		user_id:    i.get(1),
    		joined:     i.get(2),
    		came_out:   i.get(3),
    		message:    i.get(4),
    		created_at: i.get(5),
    	}
	)
    .collect::<Vec<PublicChat>>();
    Ok(r)
}

pub async fn ssc_dialogue(
	pool: PgPool,
	to_user: i32,
) -> Result<Vec<ToDialogue>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };

	let result = pg.query(
		"SELECT to_user, additionally, completed FROM subscriptions WHERE to_user=$1 AND completed=$2;",
		&[&to_user, &true]
	)
	.await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let r: Vec<ToDialogue> = rows.into_iter().map(
    	|i| ToDialogue {
    		to_user: 	  i.get(0),
    		additionally: i.get(1),
    		completed: 	  i.get(2),
    	}
	)
    .collect::<Vec<ToDialogue>>();
    Ok(r)
}

pub async fn to_room(
	pool: PgPool,
	room: String,
) -> Result<Vec<Room>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
	let result = pg.query(
		"SELECT id,user_id,joined,came_out,message,room,created_at FROM chat_room WHERE room=$1;",
		&[&room]
	)
	.await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut r: Vec<Room> = vec![];
    for i in rows {
        r.push(Room {
			id: 		i.get(0),
			user_id: 	i.get(1),
			joined: 	i.get(2),
			came_out: 	i.get(3),
			message:  	i.get(4),
			room: 		i.get(5),
			created_at: i.get(6),
        })
    }
    // let r: Vec<Room> = rows.into_iter().map(
    // 	|i| Room {
    // 		id: 		i.get(0),
    // 		user_id: 	i.get(1),
    // 		joined: 	i.get(2),
    // 		came_out: 	i.get(3),
    // 		message:  	i.get(4),
    // 		room: 		i.get(5),
    // 		created_at: i.get(6),
    // 	}
	// )
    // .collect::<Vec<Room>>();
    Ok(r)
}

pub async fn to_dialogue(
	pool: PgPool,
	dialogue: String,
) -> DialogueChat {

    let pg = pool.get().await.unwrap();

	let result = pg.query(
		"SELECT user_id, to_user FROM subscriptions WHERE dialogue=$1;",
		&[&dialogue]
	)
	.await
	.unwrap();
	let i = &result[0];
	DialogueChat {
		user_id: i.get(0),
		to_user: i.get(1),
	}
}

pub async fn total_dialogue(
	pool: PgPool,
	user_id: i32,
) -> i64 {

    let pg = pool.get().await.unwrap();
	let r = pg.query_scalar(
		"SELECT COUNT(*) FROM chat_room WHERE user_id=$1;",
		&[&user_id]
	)
	.await
	.unwrap();
	r[0]
}

pub async fn user_id_dialogue(
	pool: PgPool,
	user_id: i32, limit: i64, offset: i64
) -> Result<Vec<Room>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };

	let result = pg.query(
		"SELECT * FROM chat_room WHERE user_id=$1 ORDER BY id LIMIT $2 OFFSET $3",
		&[&user_id, &limit, &offset]
	)
	.await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let r: Vec<Room> = rows.into_iter().map(
    	|i| Room {
    		id: 		i.get(0),
    		user_id: 	i.get(1),
    		joined: 	i.get(2),
    		came_out: 	i.get(3),
    		message:  	i.get(4),
    		room: 		i.get(5),
    		created_at: i.get(6),
    	}
	)
    .collect::<Vec<Room>>();
    Ok(r)
}

pub async fn vec_del_dialogue(
	pool: PgPool,
	id: Vec<i32>,
	user_id: i32
) -> bool {

    let pg = pool.get().await.unwrap();

	for i in id {
		let result = pg.execute(
			"DELETE FROM chat_room WHERE id=$1 AND user_id=$2",
			&[&i, &user_id]
		)
		.await;
		match result {
			Err(err) => {
				println!("Err DELETE: {}", err);
				return false;
			}
			Ok(_expr) => {
				println!("DELETE number: {} has been deleted.", i);
			}
		}
	}
	true
}

pub async fn del_dialogue(
	pool: PgPool,
	id: i32, user_id: i32
) -> bool {

    let pg = pool.get().await.unwrap();

	let result = pg.execute(
		"DELETE FROM chat_room WHERE id=$1 AND user_id=$2",
		&[&id, &user_id]
	)
	.await;
	match result {
		Err(err) => {
			println!("Err DELETE: {}\n", err);
			return false;
		}
		Ok(_expr) => {
			println!("DELETE number: {} has been deleted.", id);
			//println!("Amount deleted: {}", expr.rows_affected());
		}
	}
	true
}
