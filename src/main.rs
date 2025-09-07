#![allow(unused)]

use axum::{
    Json, Router, debug_handler,
    extract::{Path, Query},
    http::{Method, Uri},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
};
use serde_json::json;
use std::net::SocketAddr;
use ticket::{
    ctx::Ctx,
    error::{Error, Result},
    log::log_request,
    web::mw_auth::mw_ctx_resolver,
};
use uuid::Uuid;

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

    let uuid = Uuid::new_v4();

    // Get the eventual response error
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|e| e.client_status_and_error());
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string()
                }
            });
            println!("    ->> client_error_body: {client_error_body}");
            (*status_code, Json(client_error_body)).into_response()
        });
    // Build and log the server log line.
    println!("    ->> server log line - {uuid} - Error: {service_error:?}");
    let client_error = client_status_error.map(|(_, client_error)| client_error);

    // For logging, we need to extract request information differently
    // This is a simplified version since we can't access request details here
    log_request(
        uuid,
        Method::GET,
        "/".parse().unwrap(),
        None,
        service_error,
        client_error,
    )
    .await;

    println!();

    error_response.unwrap_or(res)
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
        .layer(middleware::from_fn_with_state(mc.clone(), mw_ctx_resolver))
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
