use axum::{
    body::Body,
    http::{Response, StatusCode},
};

use headers::Cookie;


pub async fn add_msg(
    messege: String, m_alert: String, url: String
) -> Response<Body> {

    let token = messege + "," + &m_alert;

    Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", &url)
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
    cookie: Cookie
) -> Result<Option<Vec<String>>, Option<String>> {

    let token = match cookie.get("to_msg") {
        Some(expr) => expr,
        None => ""
    };

    let v: Vec<&str> = token.split(",").collect();
    let mut vec = Vec::new();
    for i in v {
        vec.push(i.to_string());
    }
    Ok(Some(vec))
}
