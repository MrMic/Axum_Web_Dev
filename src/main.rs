#![allow(unused)] // For beginning only.

/*
 *             ┌──────────┐        ┌───────────────────┐
 *             │axum:serve├──────► │routes_all (Router)│
 *             └──────────┘        └───────────────────┘
 *
 */

pub use self::error::{Error, Result};

use crate::model::ModelController;

use axum::{
	extract::{Path, Query},
	middleware,
	response::{Html, IntoResponse, Response},
	routing::{get, get_service},
	Router, ServiceExt,
};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod error;
mod model;
mod web;

// NOTE: ╾──────────────────────────────╼ MAIN ╾───────────────────────────╼
#[tokio::main]
async fn main() -> Result<()> {
	// ╾──────────────────────────╼ SETUP ╾───────────────────────╼
	let mc = ModelController::new().await?;

	let routes_apis = web::routes_tickets::routes(mc.clone())
		.route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

	let routes_all = Router::new()
		.merge(routes_hello())
		.merge(web::routes_login::routes())
		.layer(middleware::map_response(main_response_mapper))
		.nest("/api", routes_apis)
		.layer(CookieManagerLayer::new())
		.fallback_service(routes_static());

	// ╾──────────────────────────╼ START SERVER ╾───────────────────────╼
	let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
	println!("--> LISTENING on {:?}\n", listener.local_addr());
	axum::serve(listener, routes_all.into_make_service())
		.await
		.unwrap();

	Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
	println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

	println!();
	res
}

// NOTE: ═══════════════════════════════ ROUTES ════════════════════════════
fn routes_static() -> Router {
	Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// ____________________________ ROUTES_HELLO ____________________________
fn routes_hello() -> Router {
	Router::new()
		.route("/hello", get(handler_hello))
		.route("/hello2/:name", get(handler_hello2))
}

// ______________________________________________________________________
#[derive(Debug, Deserialize)]
struct HelloParams {
	name: Option<String>,
}

// NOTE: ═══════════════════════════════ HANDLERS ════════════════════════
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
