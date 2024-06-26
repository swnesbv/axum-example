use axum::{
    body::Body,
    extract::{Form, State},
    http::{Response, StatusCode},
    response::{Html, IntoResponse, Redirect},
    Extension,
};
use chrono::Utc;

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use tera::Context;

use axum_extra::TypedHeader;
use headers::Cookie;

use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Pbkdf2,
};

use crate::{auth, schema};
use crate::{
    common::{Pool, Templates},
    profile::models::{
        FormNewUser, FormPasswordChange, FormUpdateUser, NewUser, PasswordChange, UpdateUser,
    },
};

pub use axum_macros::debug_handler;

pub async fn get_signup(Extension(templates): Extension<Templates>) -> impl IntoResponse {
    Html(templates.render("signup", &Context::new()).unwrap())
}

#[debug_handler]
pub async fn post_signup(
    State(pool): State<Pool>,
    Extension(templates): Extension<Templates>,
    Form(form): Form<FormNewUser>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let mut conn = pool.get().await.unwrap();

    let salt = SaltString::generate(&mut OsRng);
    let pass = Pbkdf2.hash_password(form.password.as_bytes(), &salt);
    let hashed_password = match pass {
        Ok(pass) => pass.to_string(),
        Err(_) => "Err password".to_string(),
    };

    use schema::users::dsl::*;

    let in_names = users
        .filter(username.eq(form.username.clone()))
        .select(username)
        .first::<String>(&mut conn)
        .await
        .optional()
        .unwrap();

    let in_emails = users
        .filter(email.eq(form.email.clone()))
        .select(email)
        .first::<String>(&mut conn)
        .await
        .optional()
        .unwrap();

    let mut context = Context::new();

    if in_names == Some(form.username.clone()) {
        context.insert("for_name", "name already exists..");
        return Err(Html(templates.render("signup", &context).unwrap()));
    }
    if in_emails == Some(form.email.clone()) {
        context.insert("for_email", "email already exists..");
        return Err(Html(templates.render("signup", &context).unwrap()));
    }

    let new_user = NewUser {
        email: form.email.clone(),
        username: form.username.clone(),
        password: hashed_password,
        created_at: Utc::now(),
    };

    let _ = diesel::insert_into(users)
        .values(new_user)
        .returning(NewUser::as_returning())
        .get_result(&mut conn)
        .await;

    Ok(Redirect::to("/account/users").into_response())
}

#[debug_handler]
pub async fn get_update(
    State(pool): State<Pool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let token = auth::views::request_user(TypedHeader(cookie)).await;
    let ok_token = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(_) => return Err(Redirect::to("/account/login").into_response()),
    };

    use schema::users::dsl::*;
    let mut conn = pool.get().await.unwrap();

    let user: Option<UpdateUser> = Some(
        users
            .filter(id.eq(ok_token.id))
            .select(UpdateUser::as_select())
            .first(&mut conn)
            .await
            .unwrap(),
    );

    let mut context = Context::new();
    if user.is_some() {
        context.insert("user", &user);
        Ok(Html(templates.render("update", &context).unwrap()))
    } else {
        context.insert("not_user", "None..");
        Ok(Html(templates.render("update", &context).unwrap()))
    }
}

#[debug_handler]
pub async fn post_update_user(
    State(pool): State<Pool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Form(form): Form<FormUpdateUser>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let token = auth::views::request_user(TypedHeader(cookie)).await;

    let ok_token = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(_) => return Err(Redirect::to("/account/login").into_response()),
    };

    let mut conn = pool.get().await.unwrap();
    use schema::users::dsl::*;

    let up_user = UpdateUser {
        email: form.email.clone(),
        username: form.username.clone(),
        updated_at: Some(Utc::now()),
    };

    let _ = diesel::update(users.filter(id.eq(ok_token.id)))
        .set(up_user)
        .execute(&mut conn)
        .await;

    Ok(Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", "/account/login")
        .header(
            "Set-Cookie",
            format!(
                "{}={}; Path={}; HttpOnly={}; SameSite={}",
                "visit", "_", "/", "true", "lax",
            ),
        )
        .body(Body::from("not found"))
        .unwrap())
}

#[debug_handler]
pub async fn get_password_change(
    State(pool): State<Pool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let token = auth::views::request_user(TypedHeader(cookie)).await;

    let ok_token = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(_) => return Err(Redirect::to("/account/login").into_response()),
    };

    use schema::users::dsl::*;
    let mut conn = pool.get().await.unwrap();

    let user: Option<UpdateUser> = Some(
        users
            .filter(id.eq(ok_token.id))
            .select(UpdateUser::as_select())
            .first(&mut conn)
            .await
            .unwrap(),
    );

    let mut context = Context::new();
    if user.is_some() {
        context.insert("user", &user);
        Ok(Html(templates.render("password_change", &context).unwrap()))
    } else {
        context.insert("not_user", "None..");
        Ok(Html(templates.render("password_change", &context).unwrap()))
    }
}

#[debug_handler]
pub async fn post_password_change(
    State(pool): State<Pool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Form(form): Form<FormPasswordChange>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let token = auth::views::request_user(TypedHeader(cookie)).await;

    let ok_token = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(_) => return Err(Redirect::to("/account/login").into_response()),
    };

    let mut conn = pool.get().await.unwrap();

    let salt = SaltString::generate(&mut OsRng);
    let pass = Pbkdf2.hash_password(form.password.as_bytes(), &salt);
    let hashed_password = match pass {
        Ok(pass) => pass.to_string(),
        Err(_) => "Err password".to_string(),
    };

    let up_user = PasswordChange {
        password: hashed_password,
        updated_at: Some(Utc::now()),
    };

    use schema::users::dsl::*;
    let _ = diesel::update(users.filter(id.eq(ok_token.id)))
        .set(up_user)
        .execute(&mut conn)
        .await;

    Ok(Redirect::to("/account/login").into_response())
}
