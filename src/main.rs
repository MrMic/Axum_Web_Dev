#![allow(unused)] // For beginning only.

use axum::{response::Html, routing::get, Router, ServiceExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let routes_hello =
        Router::new().route("/hello", get(|| async { Html("<strong>Hello World!") }));

    // ╾──────────────────────────╼ START SERVER ╾───────────────────────╼
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    println!("--> lISTENING on {:?}\n", listener.local_addr());

    axum::serve(listener, routes_hello.into_make_service())
        .await
        .unwrap();

    // Ok(())
}
