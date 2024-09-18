use sqlx::postgres::PgPool;

use chrono::Utc;

use axum_extra::TypedHeader;
use headers::Cookie;

use axum::{
    body::Body,
    extract::{Multipart, State},
    http::{Response, StatusCode},
    response::{Html, IntoResponse, Redirect},
    Extension,
};

use tera::Context;

use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Pbkdf2,
};

use crate::{
    auth,
    common::Templates,
    import_export::models::{ExCsvUser},
    import_export::views::all,
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
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let token = auth::views::request_user(cookie).await;
    let cls = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(_) => return Err(Redirect::to("/account/login").into_response()),
    };
    if cls.status.contains(&"admin".to_owned()) {
        Ok(Html(templates.render("export_csv", &Context::new()).unwrap()))
    } else {
        Err(Redirect::to("/account/login").into_response())
    }
}


pub async fn export_users(
    State(pool): State<PgPool>,
    Extension(templates): Extension<Templates>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut context = Context::new();

    let token = auth::views::request_user(cookie).await;
    let cls = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Ok({
            context.insert("err_token", "token None");
            Html(templates.render("export_csv", &context).unwrap())
        }),
        Err(err) => return Err({
            context.insert("err_token", &err.expect("REASON").to_string());
            Html(templates.render("export_csv", &context).unwrap())
        }),
    };

    if cls.status.contains(&"admin".to_owned()) {
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

                let status = vec![i.status];
                let result = sqlx::query(
                    "INSERT INTO users(email, username, password, img, status, created_at)  VALUES ($1,$2,$3,$4,$5,$6)",
                    )
                    .bind(i.email)
                    .bind(i.username)
                    .bind(hashed_password)
                    .bind(i.img)
                    .bind(status)
                    .bind(Utc::now())
                    .execute(&pool)
                    .await;

                if let Err(err) = result {
                    println!("Error message: [{}].\n", err);
                    context.insert("not_details", &err.to_string());
                    return Err(Html(templates.render("export_csv", &context).unwrap()));
                };
            } // for
        } // while
    }
    context.insert("csv_details", "Ok! Export csv..");
    Ok(Html(templates.render("export_csv", &context).unwrap()))
}
