use axum::{
    extract::{Form, Path, State},
    response::{Html, IntoResponse, Redirect},
    // http::{Request,Response,StatusCode},
    // body::Body,
    Extension,
};
use chrono::{NaiveDate, Utc};

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use tera::Context;

use axum_extra::TypedHeader;
use headers::Cookie;

use crate::{auth, schema};
use crate::{
    common::{Pool, Templates},
    provision::models::{
        FormPrD, AllPrD,
        NewPrD,
        UpPrD},
};

pub use axum_macros::debug_handler;

pub async fn get_creat_days(
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let token = auth::views::request_user(TypedHeader(cookie)).await;
    let _ = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(_) => return Err(Redirect::to("/account/login").into_response()),
    };
    Ok(Html(
        templates.render("creat_days", &Context::new()).unwrap(),
    ))
}


#[debug_handler]
pub async fn post_creat_days(
    State(pool): State<Pool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Form(form): Form<FormPrD>,
) -> impl IntoResponse {

    let token = auth::views::request_token(TypedHeader(cookie)).await;

    let s_value = form.st_date.as_deref().unwrap_or("default string");
    let e_value = form.en_date.as_deref().unwrap_or("default string");

    let start: Option<NaiveDate>;
    let end: Option<NaiveDate>;

    if !s_value.is_empty() {
        start = Some(NaiveDate::parse_from_str(s_value, "%Y-%m-%d").expect("REASON"));
    } else {
        start = None
    }
    if !e_value.is_empty() {
        end = Some(NaiveDate::parse_from_str(e_value, "%Y-%m-%d").expect("REASON"));
    } else {
        end = None
    }

    let mut conn = pool.get().await.unwrap();
    use schema::provision_d::dsl::*;

    let prv = NewPrD {
        user_id: token.clone().unwrap().claims.id,
        title: form.title.clone(),
        description: form.description.clone(),
        st_date: start,
        en_date: end,
        created_at: Utc::now(),
    };
    let _ = diesel::insert_into(provision_d)
        .values(prv)
        .returning(NewPrD::as_returning())
        .get_result(&mut conn)
        .await
        .unwrap();
    Redirect::to("/").into_response()
}


/*#[debug_handler]
pub async fn post_creat_days(
    State(pool): State<Pool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Form(form): Form<FormPrD>,
) -> impl IntoResponse {

    let token = auth::views::request_token(TypedHeader(cookie)).await;

    let s_value = form.st_date.as_deref().unwrap_or("default string");
    let e_value = form.en_date.as_deref().unwrap_or("default string");
    let start = NaiveDate::parse_from_str(s_value, "%Y-%m-%d");
    let end = NaiveDate::parse_from_str(e_value, "%Y-%m-%d");

    let mut conn = pool.get().await.unwrap();
    use schema::provision_d::dsl::*;

    if s_value.is_empty() && e_value.is_empty() {
        let no_prv = NewPrD {
            user_id: token.clone().unwrap().claims.id,
            title: form.title.clone(),
            description: form.description.clone(),
            st_date: None,
            en_date: None,
            created_at: Utc::now(),
        };
        let _ = diesel::insert_into(provision_d)
            .values(no_prv)
            .returning(NewPrD::as_returning())
            .get_result(&mut conn)
            .await;
    } else if !s_value.is_empty() && e_value.is_empty() {
        let st_prv = NewPrD {
            user_id: token.clone().unwrap().claims.id,
            title: form.title.clone(),
            description: form.description.clone(),
            st_date: Some(start.expect("REASON")),
            en_date: None,
            created_at: Utc::now(),
        };
        let _ = diesel::insert_into(provision_d)
            .values(st_prv)
            .returning(NewPrD::as_returning())
            .get_result(&mut conn)
            .await;
    } else if s_value.is_empty() && !e_value.is_empty() {
        let en_prv = NewPrD {
            user_id: token.clone().unwrap().claims.id,
            title: form.title.clone(),
            description: form.description.clone(),
            st_date: None,
            en_date: Some(end.expect("REASON")),
            created_at: Utc::now(),
        };
        let _ = diesel::insert_into(provision_d)
            .values(en_prv)
            .returning(NewPrD::as_returning())
            .get_result(&mut conn)
            .await;
    } else {
        let prv = NewPrD {
            user_id: token.clone().unwrap().claims.id,
            title: form.title.clone(),
            description: form.description.clone(),
            st_date: Some(start.expect("REASON")),
            en_date: Some(end.expect("REASON")),
            created_at: Utc::now(),
        };
        let _ = diesel::insert_into(provision_d)
            .values(prv)
            .returning(NewPrD::as_returning())
            .get_result(&mut conn)
            .await;
    }
    Redirect::to("/").into_response()
}*/

#[debug_handler]
pub async fn get_update_days(
    Path(prv_id): Path<String>,
    State(pool): State<Pool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let token = auth::views::request_user(TypedHeader(cookie)).await;
    let _ = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(_) => return Err(Redirect::to("/account/login").into_response()),
    };

    let number: i32 = prv_id.parse().expect("Not a valid number");

    let mut conn = pool.get().await.unwrap();
    use schema::provision_d::dsl::*;

    let i: Option<AllPrD> = Some(
        provision_d
            .filter(id.eq(number))
            .select(AllPrD::as_select())
            .first(&mut conn)
            .await
            .unwrap(),
    );

    let mut context = Context::new();
    context.insert("i", &i);
    Ok(Html(templates.render("update_dates", &context).unwrap()))
}

#[debug_handler]
pub async fn post_update_days(
    Path(prv_id): Path<String>,
    State(pool): State<Pool>,
    Form(form): Form<FormPrD>,
) -> impl IntoResponse {
    
    let s_value = form.st_date.as_deref().unwrap_or("default string");
    let e_value = form.en_date.as_deref().unwrap_or("default string");
    let start = NaiveDate::parse_from_str(s_value, "%Y-%m-%d");
    let end = NaiveDate::parse_from_str(e_value, "%Y-%m-%d");

    let number: i32 = prv_id.parse().expect("Not a valid number");
    let mut conn = pool.get().await.unwrap();
    use schema::provision_d::dsl::*;

    let i: UpPrD = provision_d
        .filter(id.eq(number))
        .select(UpPrD::as_select())
        .first(&mut conn)
        .await
        .unwrap();

    if s_value.is_empty() && e_value.is_empty() {
        let no_prv = UpPrD {
            title: form.title.clone(),
            description: form.description.clone(),
            st_date: i.st_date,
            en_date: i.en_date,
            updated_at: Some(Utc::now()),
        };
        let _ = diesel::update(provision_d.filter(id.eq(number)))
            .set(no_prv)
            .execute(&mut conn)
            .await;
    } else if !s_value.is_empty() && e_value.is_empty() {
        let st_prv = UpPrD {
            title: form.title.clone(),
            description: form.description.clone(),
            st_date: Some(start.expect("REASON")),
            en_date: i.en_date,
            updated_at: Some(Utc::now()),
        };
        let _ = diesel::update(provision_d.filter(id.eq(number)))
            .set(st_prv)
            .execute(&mut conn)
            .await;
    } else if s_value.is_empty() && !e_value.is_empty() {
        let en_prv = UpPrD {
            title: form.title.clone(),
            description: form.description.clone(),
            st_date: i.st_date,
            en_date: Some(end.expect("REASON")),
            updated_at: Some(Utc::now()),
        };
        let _ = diesel::update(provision_d.filter(id.eq(number)))
            .set(en_prv)
            .execute(&mut conn)
            .await;
    } else {
        let prv = UpPrD {
            title: form.title.clone(),
            description: form.description.clone(),
            st_date: Some(start.expect("REASON")),
            en_date: Some(end.expect("REASON")),
            updated_at: Some(Utc::now()),
        };
        let _ = diesel::update(provision_d.filter(id.eq(number)))
            .set(prv)
            .execute(&mut conn)
            .await;
    }
    Redirect::to("/").into_response()
}
