use axum::{
    body::Body,
    http::{Response, StatusCode},
};

use axum_extra::TypedHeader;
use headers::Cookie;

use crate::{
    photo::models::Msg,
};


pub async fn add_msg(
    messege: String,
    m_alert: String,
	url: String,
) -> Response<Body> {

    let entity = Msg {
        msg: messege,
        alert: m_alert,
    };

    use bincode;
    let encoded: Vec<u8> = bincode::serialize(&entity).unwrap();
    use base64::{engine::general_purpose::STANDARD, Engine as _};
    let token = STANDARD.encode(encoded);

    Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", url.clone())
        .header(
            "Set-Cookie",
            format!(
                "{}={}; Path={}; HttpOnly={}; SameSite={}; Max-Age={};",
                "to_msg", token, url, "true", "lax", 60
            ),
        )
        .body(Body::from("not found"))
        .unwrap()
}

pub async fn read_msg(
    cookie: TypedHeader<Cookie>,
) -> Result<Option<Msg>, Option<String>> {

    let token = match cookie.get("to_msg") {
        Some(expr) => expr,
        None => return Ok(Some(Msg { msg: "".to_string(), alert: "".to_string() })),
    };

    use base64::{engine::general_purpose::STANDARD, Engine as _};

    let t_64 = match STANDARD.decode(token) {
        Ok(claims) => claims,
        Err(err) => err.to_string().into(),
    };
    let k: Msg = bincode::deserialize(&t_64[..]).unwrap();
    Ok(Some(k))
}