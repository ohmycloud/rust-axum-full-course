use axum::{
    Json, Router,
    response::{IntoResponse, Response},
    routing::post,
};
use serde::Deserialize;
use serde_json::json;
use tower_cookies::{Cookie, Cookies};

use crate::{Error, web};

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, Json(payload): Json<LoginPayload>) -> Response {
    println!("->> {:<12} - api_login", "HANDLER");
    if payload.username != "admin" || payload.password != "admin" {
        return Error::LoginFail.into_response();
    }

    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    body.into_response()
}
