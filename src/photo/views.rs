use axum::{
    body::Body,
    http::{Response, StatusCode},
};

pub async fn add_msg(
    err: String, alert: String, url: String
) -> Response<Body> {

    let token = err + "," + &alert;
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


