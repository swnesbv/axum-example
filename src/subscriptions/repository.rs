use crate::{
	common::{PgPool},
	subscriptions::models::{CheckSsc, Subscription, Group}
};


pub async fn check_ssc(
    pool: PgPool,
) -> Result<Vec<CheckSsc>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let result = pg.query(
        "SELECT id,user_id,to_user,completed FROM subscriptions;", &[]
    )
    .await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut r: Vec<CheckSsc> = vec![];
    for i in rows {
        r.push(CheckSsc {
            id:           i.get("id"),
            user_id:      i.get("user_id"),
            to_user:      i.get("to_user"),
            completed:    i.get("completed"),
        })
    }
    Ok(r)
}


pub async fn ssc_owner(
	pool: PgPool,
	user_id: i32,
) -> Result<Vec<Subscription>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
	let result = pg.query(
		"SELECT id,user_id,title,description,to_user,to_group,dialogue,additionally,completed,created_at,updated_at FROM subscriptions WHERE user_id=$1;", &[&user_id]
	)
	.await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let mut r: Vec<Subscription> = vec![];
    for i in rows {
        r.push(Subscription {
            id:           i.get("id"),
            user_id:      i.get("user_id"),
            title:        i.get("title"),
            description:  i.get("description"),
            to_user:      i.get("to_user"),
            to_group:     i.get("to_group"),
            dialogue:     i.get("dialogue"),
            additionally: i.get("additionally"),
            completed:    i.get("completed"),
            created_at:   i.get("created_at"),
            updated_at:   i.get("updated_at")
        })
    }

    // let r: Vec<Subscription> = rows.into_iter().map(
    // 	|i| Subscription {
    // 		id:           i.get(0),
    // 		user_id:      i.get(1),
    // 		title:        i.get(2),
    // 		description:  i.get(3),
    // 		to_user:      i.get(4),
    // 		to_group:     i.get(5),
    // 		dialogue:     i.get(6),
    //         additionally: i.get(7),
    // 		completed:    i.get(8),
    // 		created_at:   i.get(9),
    // 		updated_at:   i.get(10)
    // 	}
	// )
    // .collect::<Vec<Subscription>>();

    Ok(r)
}

pub async fn ssc_to_user(
	pool: PgPool,
	user_id: i32,
) -> Result<Vec<Subscription>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
	let result = pg.query(
		"SELECT * FROM subscriptions WHERE to_user=$1;",
		&[&user_id]
	)
	.await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let r: Vec<Subscription> = rows.into_iter().map(
    	|i| Subscription {
    		id:           i.get(0),
    		user_id:      i.get(1),
    		title:        i.get(2),
    		description:  i.get(3),
    		to_user:      i.get(4),
    		to_group:     i.get(5),
    		dialogue:     i.get(6),
    		additionally: i.get(7),
    		completed:    i.get(8),
    		created_at:   i.get(9),
    		updated_at:   i.get(10)
    	}
	)
    .collect::<Vec<Subscription>>();
    Ok(r)
}

pub async fn to_ssc_user(
	pool: PgPool,
	to_user: i32,
) -> Result<Vec<Subscription>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
	let result = pg.query(
		"SELECT * FROM subscriptions WHERE to_user=$1 AND completed=$2;",
		&[&to_user, &false]
	)
	.await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let r: Vec<Subscription> = rows.into_iter().map(
    	|i| Subscription {
    		id:           i.get(0),
    		user_id:      i.get(1),
    		title:        i.get(2),
    		description:  i.get(3),
    		to_user:      i.get(4),
    		to_group:     i.get(5),
    		dialogue:     i.get(6),
    		additionally: i.get(7),
    		completed:    i.get(8),
    		created_at:   i.get(9),
    		updated_at:   i.get(10)
    	}
	)
    .collect::<Vec<Subscription>>();
    Ok(r)
}


pub async fn all_groups(
	pool: PgPool
) -> Result<Vec<Group>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
	let result = pg.query(
		"SELECT * FROM groups;", &[]
	)
	.await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let r: Vec<Group> = rows.into_iter().map(
    	|i| Group {
    		id:          i.get(0),
    		user_id:     i.get(1),
    		title:       i.get(2),
    		description: i.get(3),
    		img:         i.get(4),
    		completed:   i.get(5),
    		created_at:  i.get(6),
    		updated_at:  i.get(7)
    	}
	)
    .collect::<Vec<Group>>();
    Ok(r)
}

pub async fn all_groups_user(
	pool: PgPool,
	user_id: i32,
) -> Result<Vec<Group>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
	let result = pg.query(
		"SELECT * FROM groups WHERE user_id=$1;",
		&[&user_id]
	)
	.await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let r: Vec<Group> = rows.into_iter().map(
    	|i| Group {
    		id:          i.get(0),
    		user_id:     i.get(1),
    		title:       i.get(2),
    		description: i.get(3),
    		img:         i.get(4),
    		completed:   i.get(5),
    		created_at:  i.get(6),
    		updated_at:  i.get(7)
    	}
	)
    .collect::<Vec<Group>>();
    Ok(r)
}

pub async fn to_ssc_group(
	pool: PgPool,
	to_group: i32,
) -> Result<Vec<Subscription>, Option<String>> {

    let pg = match pool.get().await{
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
	let result = pg.query(
		"SELECT * FROM subscriptions WHERE to_group=$1 AND completed=$2;",
		&[&to_group, &false]
	)
	.await;
    let rows = match result {
        Ok(expr) => expr,
        Err(err) => return Err(Some(err.to_string()))
    };
    let r: Vec<Subscription> = rows.into_iter().map(
    	|i| Subscription {
    		id:           i.get(0),
    		user_id:      i.get(1),
    		title:        i.get(2),
    		description:  i.get(3),
    		to_user:      i.get(4),
    		to_group:     i.get(5),
    		dialogue:     i.get(6),
    		additionally: i.get(7),
    		completed:    i.get(8),
    		created_at:   i.get(9),
    		updated_at:   i.get(10)
    	}
	)
    .collect::<Vec<Subscription>>();
    Ok(r)
}