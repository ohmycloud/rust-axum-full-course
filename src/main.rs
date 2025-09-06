#![allow(unused)]

use std::net::SocketAddr;

use anyhow::Ok;
use axum::{
    Router,
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
};
use serde::Deserialize;

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
        .route("/hello", get(hander_hello))
        .route("/greet/{name}", get(handler_greet));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("->> LISTENING on http://{addr}\n");

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
