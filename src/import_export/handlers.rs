use std::sync::Arc;
use axum::{
    body::Body,
    extract::{Multipart, State},
    http::{Response, StatusCode},
    http::{header::{HeaderMap}},
    response::{Html, IntoResponse},
    Extension,
};
use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Pbkdf2,
};
use tera::Context;

use crate::{
    common::Templates,
    auth::models::{AuthRedis},
    import_export::views::{all, w_status},
    import_export::models::{ExCsvUser},
};

pub async fn import_users(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> impl IntoResponse {

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            let mut context = Context::new();
            context.insert("err", &err);
            return Err(Html(templates.render("export_csv", &context).unwrap()))
        }
        Ok(None) | Err(None) => {
            let mut context = Context::new();
            context.insert("err", "Caramba bullfighting and damn it token..!");
            return Err(Html(templates.render("export_csv", &context).unwrap()))
        }
    };

    if t.status.contains(&"admin".to_owned()) {

        let mut wtr = csv::WriterBuilder::new()
            .has_headers(false)
            .from_writer(vec![]);

        wtr.write_record(["id","email","username","password","img","status","created_at","updated_at"]).unwrap();
        let data = all(i.pool.clone()).await.unwrap();
        for w in data {
            wtr.write_record(&[
                w.id.to_string(),
                w.email.to_string(),
                w.username.to_string(),
                w.password.to_string(),
                w.img.unwrap_or_default(),
                w_status(w.status),
                w.created_at.to_string(),
                format!("{:?}", w.updated_at),
            ]).unwrap();
        }
        wtr.flush().unwrap();
        Ok(
            Response::builder()
                .status(StatusCode::OK)
                .header("Location", "/account/users")
                .header("Content-Disposition", "attachment;filename=or.csv")
                .body(Body::from(wtr.into_inner().unwrap()))
                .unwrap()
        )
    } else {
        let mut context = Context::new();
        context.insert("err", "Caramba bullfighting and damn it (import users admin)..!");
        Err(Html(templates.render("export_csv", &context).unwrap()))
    }
}

pub async fn get_export_users(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            let mut context = Context::new();
            context.insert("err", &err);
            return Err(Html(templates.render("export_csv", &context).unwrap()))
        }
        Ok(None) | Err(None) => {
            let mut context = Context::new();
            context.insert("err", "Caramba bullfighting and damn it token..!");
            return Err(Html(templates.render("export_csv", &context).unwrap()))
        }
    };
    if t.status.contains(&"admin".to_owned()) {
        Ok(Html(templates.render("export_csv", &Context::new()).unwrap()))
    } else {
        let mut context = Context::new();
        context.insert("err", "Caramba bullfighting and damn it (export users admin)..!");
        Err(Html(templates.render("export_csv", &context).unwrap()))
    }
}

pub async fn post_export_users(
    headers: HeaderMap,
    State(i): State<Arc<AuthRedis>>,
    Extension(templates): Extension<Templates>,
    mut multipart: Multipart,
) -> impl IntoResponse {

    let mut context = Context::new();

    let t = match i.ctx(headers).await {
        Ok(Some(expr)) => expr,
        Err(Some(err)) => {
            context.insert("err", &err);
            return Err(Html(templates.render("export_csv", &context).unwrap()))
        }
        Ok(None) | Err(None) => {
            context.insert("err", "Caramba bullfighting and damn it token..!");
            return Err(Html(templates.render("export_csv", &context).unwrap()))
        }
    };

    if t.status.contains(&"admin".to_owned()) {
        while let Some(f) = multipart.next_field().await.unwrap() {
            let data = f.bytes().await.unwrap();
            let body = String::from_utf8(data.to_vec()).unwrap();

            let mut rdr = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(body.as_bytes());

            for result in rdr.deserialize() {
                let r: ExCsvUser = result.unwrap();
                println!(" r.. {:?}", r);

                let salt = SaltString::generate(&mut OsRng);
                let pass = Pbkdf2.hash_password(r.password.as_bytes(), &salt);
                let hashed_password = match pass {
                    Ok(pass) => pass.to_string(),
                    Err(_) => "Err password".to_string(),
                };

                let pg = match i.pool.get().await{
                    Ok(expr) => expr,
                    Err(err) => {
                        context.insert("err", &err.to_string());
                        return Err(
                            Html(templates.render("photo", &context).unwrap())
                        )
                    }
                };

                let v = vec![r.status];
                let result = pg.execute(
                    "INSERT INTO users(email, username, password, img, status, created_at) VALUES ($1,$2,$3,$4,$5,now())",
                    &[&r.email, &r.username, &hashed_password, &r.img, &v]
                ).await;

                if let Err(err) = result {
                    println!("Error message: [{}].\n", err);
                    context.insert("err", &err.to_string());
                    return Err(Html(templates.render("export_csv", &context).unwrap()));
                };
            } // for
        } // while
    }
    context.insert("csv_details", "Ok! Export csv..");
    Ok(Html(templates.render("export_csv", &context).unwrap()))
}
