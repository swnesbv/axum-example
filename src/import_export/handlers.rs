use sqlx::postgres::PgPool;

use chrono::{Utc};

use axum::{
    body::Body,
    extract::{Multipart, State},
    http::{
        // Request,
        Response,
        StatusCode,
    },
    response::{
        Html, IntoResponse,
        // Redirect
    },
    Extension,
};

use tera::Context;

use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Pbkdf2,
};

use crate::{
    common::{Templates},
    import_export::models::{ExCsvUser, ExCsv},
    import_export::views::{all},
};


pub async fn import_users(State(pool): State<PgPool>) -> impl IntoResponse {

    let data = all(pool).await.unwrap();

    let mut wtr = csv::Writer::from_writer(vec![]);

    for i in data {
        wtr.serialize(i).unwrap();
    }
    wtr.flush().unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .header("Location", "/account/users")
        .header("Content-Disposition", "attachment;filename=or.csv")
        .body(Body::from(wtr.into_inner().unwrap()))
        .unwrap()
}

// export

pub async fn get_export_users(
    Extension(templates): Extension<Templates>
) -> impl IntoResponse {

    Html(templates.render("export_csv", &Context::new()).unwrap())
}


pub async fn export_users(
    State(pool): State<PgPool>,
    Extension(templates): Extension<Templates>,
    mut multipart: Multipart,
) -> impl IntoResponse {

    let mut context = Context::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let data = field.bytes().await.unwrap();

        let body = String::from_utf8(data.to_vec()).unwrap();

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(body.as_bytes());

        for result in rdr.deserialize() {
            let i: ExCsvUser = result.unwrap();
            println!(" i.. {:?}", i);

            let salt = SaltString::generate(&mut OsRng);
            let pass = Pbkdf2.hash_password(i.password.as_bytes(), &salt);
            let hashed_password = match pass {
                Ok(pass) => pass.to_string(),
                Err(_) => "Err password".to_string(),
            };

            let u = ExCsv {
                email: i.email,
                username: i.username,
                password: hashed_password,
                img: i.img,
                created_at: Utc::now(),
                updated_at: Some(Utc::now()),
            };

            let result = sqlx::query(
                "INSERT INTO users(email, username, password, img, created_at)  VALUES ($1,$2,$3,$4,$5)", 
                )
                .bind(&u.email)
                .bind(&u.username)
                .bind(&u.password)
                .bind(&u.img)
                .bind(u.created_at)
                .execute(&pool)
                .await;

            if let Err(err) = result {
                println!("Error inserting employee: {:#?}", u);
                println!("Error message: [{}].\n", err);
                context.insert("not_details", &err.to_string());
                return Err(Html(templates.render("export_csv", &context).unwrap()));
            };
        } // for
    } // while

    context.insert("csv_details", "Ok! Export csv..");
    Ok(Html(templates.render("export_csv", &context).unwrap()))
}
