#![allow(unused)] // For beginning only.

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router, ServiceExt,
};
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
async fn handler_hello() -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");

    Html("Hello, <strong>World!</strong>")
}
