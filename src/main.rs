#![allow(unused)] // For beginning only.

use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
    Router, ServiceExt,
};
use serde::Deserialize;
use tokio::net::TcpListener;

// ╾──────────────────────────────╼ MAIN ╾───────────────────────────╼
#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(handler_hello));

    // ╾──────────────────────────╼ START SERVER ╾───────────────────────╼
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    println!("--> LISTENING on {:?}\n", listener.local_addr());

    axum::serve(listener, routes_hello.into_make_service())
        .await
        .unwrap();

    // Ok(())
}

// ____________________________ HANDLER_HELLO ____________________________
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello<strong>{name}</strong>"))
}
