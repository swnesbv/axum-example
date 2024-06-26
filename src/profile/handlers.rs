use axum::{
    extract::Path,
    response::{Html, IntoResponse},
    Extension,
};

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use tera::Context;

use axum_extra::TypedHeader;
use headers::Cookie;

use crate::{auth, schema};
use crate::{
    common::{DBConnection, Templates},
    profile::models::ListUser,
};

pub use axum_macros::debug_handler;

#[debug_handler]
pub async fn index(
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    
    let mut context = Context::new();

    let token = auth::views::request_user(TypedHeader(cookie)).await;

    match token {
        Ok(Some(token)) => {
            context.insert("token", &token);
            Ok(Html(templates.render("index", &context).unwrap()))
        }
        Ok(None) => {
            context.insert("not_user", "unauthorized");
            Err(Html(templates.render("index", &context).unwrap()))
        }
        Err(_) => {
            context.insert("not_user", "err token");
            Err(Html(templates.render("index", &context).unwrap()))
        }
    }
}


pub async fn users(
    DBConnection(mut conn): DBConnection,
    Extension(templates): Extension<Templates>,
) -> impl IntoResponse {
    
    use schema::users::dsl::*;
    let obj = users
        .select(ListUser::as_select())
        .load(&mut conn)
        .await
        .unwrap();

    let mut context = Context::new();
    context.insert("users", &obj);
    Html(templates.render("users", &context).unwrap())
}

pub async fn user(
    Path(name): Path<String>,
    DBConnection(mut conn): DBConnection,
    Extension(templates): Extension<Templates>,
) -> impl IntoResponse {
    use schema::users::dsl::*;

    let user: Option<ListUser> = Some(
        users
            .filter(username.eq(name))
            .select(ListUser::as_select())
            .first(&mut conn)
            .await
            .unwrap(),
    );

    let mut context = Context::new();
    if user.is_some() {
        context.insert("user", &user);
        Html(templates.render("user", &context).unwrap())
    } else {
        context.insert("not_user", "None..");
        Html(templates.render("user", &context).unwrap())
    }
}
