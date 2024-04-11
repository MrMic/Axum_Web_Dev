#![allow(unused)] // For beginning only.

use axum::{
	extract::{Path, Query},
	response::{Html, IntoResponse},
	routing::get,
	Router, ServiceExt,
};
use serde::Deserialize;
use tokio::net::TcpListener;

// ╾──────────────────────────────╼ MAIN ╾───────────────────────────╼
#[tokio::main]
async fn main() {
	let routes_all = Router::new().merge(routes_hello());

	// ╾──────────────────────────╼ START SERVER ╾───────────────────────╼
	let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

	println!("--> LISTENING on {:?}\n", listener.local_addr());

	axum::serve(listener, routes_hello.into_make_service())
		.await
		.unwrap();

	// Ok(())
}

// ____________________________ ROUTES HELLO ____________________________
fn routes_hello() -> Router {
	Router::new()
		.route("/hello", get(handler_hello))
		.route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
	name: Option<String>,
}

// e.g., `/hello/?name=Mic`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
	println!("->> {:<12} - handler_hello {params:?}", "HANDLER");

	let name = params.name.as_deref().unwrap_or("World");
	Html(format!("Hello <strong>{name}</strong>"))
}

// e.g., `/handler_hello2/Mike`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
	println!("->> {:<12} - handler_hello2 {name:?}", "HANDLER");

	Html(format!("Hello2 <strong>{name}</strong>"))
}
