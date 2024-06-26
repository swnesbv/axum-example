use axum::{
    extract::{Form, Path, State},
    response::{Html, IntoResponse, Redirect},
    Extension,
};
use chrono::{Utc};

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use tera::Context;

use axum_extra::TypedHeader;
use headers::Cookie;

use crate::{auth, schema};
use crate::{
    common::{Pool, Templates},
    provision::models::{
        FormPrdBkg, AllPrD, BkgPrD, UpPrdBkg,
    },
    provision::views::{creat_bkg, update_prv, all_days},
};


pub async fn get_all_days(
    State(pool): State<Pool>,
    Extension(templates): Extension<Templates>
) -> impl IntoResponse {

    let all = all_days(State(pool)).await.unwrap();

    let mut context = Context::new();
    context.insert("all", &all);
    Html(templates.render("all_days", &context).unwrap())
}


pub async fn get_detail_days(
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
    Ok(Html(templates.render("detail_days", &context).unwrap()))
}

pub async fn post_detail_days(
    Path(prv_id): Path<String>,
    State(pool): State<Pool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Form(form): Form<FormPrdBkg>,
) -> impl IntoResponse {

    let token = auth::views::request_token(TypedHeader(cookie)).await;
    let owner = token.clone().unwrap().claims.id;

    let start = form.s_dates;
    let end = form.e_dates;

    let number: i32 = prv_id.parse().expect("Not a valid number");

    let bkg = BkgPrD {
        user_id: owner,
        provision_d_id: Some(number),
        title: form.title.clone(),
        description: form.description.clone(),
        st_date: start,
        en_date: end,
        created_at: Utc::now(),
    };

    let _ = creat_bkg(State(pool.clone()), bkg).await;

    let s_vec = vec![start];
    println!("s vec..{:?}", s_vec);

    let e_vec = vec![end];
    println!("e vec..{:?}", e_vec);

    let d_vec = vec![start, end];
    println!("d vec..{:?}", d_vec);

    let prv = UpPrdBkg {
        s_dates: Some(s_vec),
        e_dates: Some(e_vec),
        dates:   Some(d_vec),
        updated_at: Some(Utc::now()),
    };
    let _ = update_prv(State(pool), prv, number).await;

    Redirect::to("/booking/all-booking").into_response()
}
