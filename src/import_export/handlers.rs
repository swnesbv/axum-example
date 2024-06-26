use chrono::Utc;

use axum::{
    body::Body,
    extract::{Multipart, State},
    http::{
        // Request,
        Response,
        StatusCode,
    },
    response::{Html, IntoResponse, Redirect},
    Extension,
};

use tera::Context;

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use csv::{Reader, Writer};

use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Pbkdf2,
};

use crate::schema;
use crate::{
    common::{Pool, Templates},
    import_export::models::{CsvUser, ExCsvUser},
};

pub use axum_macros::debug_handler;

#[debug_handler]
pub async fn import_users(State(pool): State<Pool>) -> impl IntoResponse {
    let mut conn = pool.get().await.unwrap();

    use schema::users::dsl::*;

    let data = users
        .select(CsvUser::as_select())
        .load(&mut conn)
        .await
        .unwrap();

    let mut wtr = Writer::from_writer(vec![]);

    for pat in data {
        wtr.serialize(pat).unwrap();
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

#[debug_handler]
pub async fn get_export_users(Extension(templates): Extension<Templates>) -> impl IntoResponse {
    Html(templates.render("export_csv", &Context::new()).unwrap())
}

#[debug_handler]
pub async fn export_users(State(pool): State<Pool>, mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let data = field.bytes().await.unwrap();

        let body = String::from_utf8(data.to_vec()).unwrap();
        let mut rdr = Reader::from_reader(body.as_bytes());

        let mut conn = pool.get().await.unwrap();
        use schema::users::dsl::*;

        for result in rdr.deserialize() {
            let record: ExCsvUser = result.unwrap();
            println!("record .. {:?}", record);
            println!("result id .. {:?}", record.password);

            let salt = SaltString::generate(&mut OsRng);
            let pass = Pbkdf2.hash_password(record.password.as_bytes(), &salt);
            let hashed_password = match pass {
                Ok(pass) => pass.to_string(),
                Err(_) => "Err password".to_string(),
            };

            let new_user = CsvUser {
                id: record.id,
                email: record.email,
                username: record.username,
                password: hashed_password,
                img: record.img,
                created_at: Utc::now(),
                updated_at: Some(Utc::now()),
            };
            diesel::insert_into(users)
                .values(new_user)
                .returning(CsvUser::as_returning())
                .get_result(&mut conn)
                .await
                .unwrap();
        }
    }

    Redirect::to("/").into_response()
}

// use axum::body;
// req: Request<Body>
// let buf = body::to_bytes(
//     req.into_body(), usize::MAX
// ).await.unwrap();
// let body = String::from_utf8(buf.to_vec()).unwrap();

// let mut rdr = Reader::from_reader(body.as_bytes());
