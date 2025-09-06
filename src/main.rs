#![allow(unused)]

use std::net::SocketAddr;

use anyhow::Ok;
use axum::{
    Router,
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::{get, get_service},
};
use rust_axum_full_course::web;
use serde::Deserialize;
use tower_http::services::ServeDir;

fn routes_static() -> Router {
    Router::new().fallback_service(ServeDir::new("static"))
}

fn routes_all() -> Router {
    Router::new()
        .route("/hello", get(hander_hello))
        .route("/greet/{name}", get(handler_greet))
}

async fn hander_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}!!!</strong>"))
}

async fn handler_greet(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_greet - {name}", "HANDLER");
    Html(format!("How are you, <strong>{name}?</strong>"))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .merge(routes_all())
        .merge(web::routes_login::routes())
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("->> LISTENING on http://{addr}\n");

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
