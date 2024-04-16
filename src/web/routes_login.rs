use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{Error, Result};
// NOTE: ════════════════════════════════ ROUTES ════════════════════════════
pub fn routes() -> Router {
	Router::new().route("/api/login", post(api_login))
}

// NOTE: ═══════════════════════════════ HANDLERS ════════════════════════
async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
	println!("->> {:<12} - api_login {payload:?}", "HANDLER");

	// TODO: Implement real db/auth logic
	if payload.username != "demo1" || payload.pwd != "welcome" {
		return Err(Error::LoginFail);
	}

	// TODO: Set Cookies

	// Create the success body.
	let body = Json(json!({
		"result": {
			"success": true
		}
	}
	));

	Ok(body)
} // (Json<LoginPayload>)

// ______________________________________________________________________
#[derive(Debug, Deserialize)]
struct LoginPayload {
	username: String,
	pwd: String,
}
