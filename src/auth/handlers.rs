use axum::{
    body::Body,
    extract::{Form, State},
    http::{Response, StatusCode},
    response::{Html, IntoResponse},
    Extension,
};

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use pbkdf2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Pbkdf2,
};

use jsonwebtoken::{encode, EncodingKey, Header};

use tera::Context;

use crate::schema;
use crate::{
    auth::models::{Claims, FormLogin, ListUser},
    common::{Pool, Templates},
};

pub use axum_macros::debug_handler;

pub async fn get_login(Extension(templates): Extension<Templates>) -> impl IntoResponse {
    Html(templates.render("login", &Context::new()).unwrap())
}

#[debug_handler]
pub async fn post_login(
    State(pool): State<Pool>,
    Extension(templates): Extension<Templates>,
    Form(form): Form<FormLogin>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // ..
    use std::time::Instant;
    let start = Instant::now();
    // ..

    let mut conn = pool.get().await.unwrap();
    use schema::users::dsl::*;

    let in_emails = users
        .filter(email.eq(form.email.clone()))
        .select(email)
        .first::<String>(&mut conn)
        .await
        .optional()
        .unwrap();
    let for_pass = users
        .filter(email.eq(form.email.clone()))
        .select(password)
        .first::<String>(&mut conn)
        .await
        .optional()
        .unwrap();

    // 1..
    let end_1 = start.elapsed();
    println!(" end 1: {:.2?}", end_1);
    // ..

    let mut context = Context::new();

    if in_emails.is_none() {
        context.insert("for_email", "this email is not available..!");
        return Err(Html(templates.render("login", &context).unwrap()));
    }
    let pass = if let Some(for_pass) = for_pass {
        for_pass
    } else {
        "Error".to_string()
    };

    // 2..
    let end_2 = start.elapsed();
    println!(" end 2: {:.2?}", end_2);
    // ..

    let parsed_hash = PasswordHash::new(&pass).unwrap();

    // 3..
    let end_3 = start.elapsed();
    println!(" end 3: {:.2?}", end_3);
    // ..

    // start_veri..
    let start_veri = Instant::now();
    // ..

    let veri = Pbkdf2
        .verify_password(form.password.clone().as_bytes(), &parsed_hash)
        .is_ok();
    if !veri {
        context.insert("for_password", "password is not correct..!");
        return Err(Html(templates.render("login", &context).unwrap()));
    };

    // start_veri..
    let end_veri = start_veri.elapsed();
    println!(" end_veri: {:.2?}", end_veri);
    // 4..
    let end_4 = start.elapsed();
    println!(" end 4: {:.2?}", end_4);
    // ..

    let in_user = users
        .filter(email.eq(form.email.clone()))
        .select(ListUser::as_select())
        .first::<ListUser>(&mut conn)
        .await
        .optional()
        .unwrap();
    let user = match in_user {
        Some(user) => user,
        None => return Err(Html(templates.render("login", &context).unwrap())),
    };

    let claims = Claims {
        id: user.id,
        email: user.email,
        username: user.username,
        exp: 10000000000,
    };
    let jwt_secret = dotenv::var("JWT_SECRET").expect("JWT SECRET must be set");
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap();

    // 5..
    let end_5 = start.elapsed();
    println!(" end 5: {:.2?}", end_5);
    // ..

    Ok(Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", "/account/users")
        .header(
            "Set-Cookie",
            format!(
                "{}={}; Path={}; HttpOnly={}; SameSite={}",
                "visit", token, "/", "true", "lax",
            ),
        )
        .body(Body::from("not found"))
        .unwrap())
}
