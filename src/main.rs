#![allow(unused)]

use axum::{
    Router,
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
};
use std::net::SocketAddr;
use ticket::error::Result;

use serde::Deserialize;
use ticket::{model::ModelController, web};
use tower_cookies::CookieManagerLayer;
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

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    println!();

    res
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the ModelController
    let mc = ModelController::new().await?;

    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let app = Router::new()
        .merge(routes_all())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("->> LISTENING on http://{addr}\n");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
