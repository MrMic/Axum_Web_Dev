use std::time::SystemTime;

use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::{ctx::Ctx, error::ClientError, Error, Result};

// * NOTE: ___________________________ FUNCTIONS ___________________________
pub async fn log_request(
    uuid: Uuid,
    req_method: Method,
    uri: Uri,
    ctx: Option<Ctx>,
    service_error: Option<&Error>,
    client_error: Option<ClientError>,
) -> Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let error_type = service_error.map(|se| se.to_string());
    let error_data = serde_json::to_value(service_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|v| v.take()));

    // Create the RequestLogLine
    let log_line = RequestLogLine {
        uuid: uuid.to_string(),
        timestamp: timestamp.to_string(),

        user_id: ctx.map(|c| c.user_id()),

        req_path: uri.to_string(),
        req_method: req_method.to_string(),

        client_error_type: client_error.as_ref().map(|ce| ce.as_ref().to_string()),
        error_type,
        error_data,
    };

    println!("    ->> log_request: \n{}", json!(log_line));

    // * TODO: Send to cloud-watch

    Ok(())
}

// * NOTE: ___________________________ VARIABLES ___________________________
#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    uuid: String,      // uuid string formatted
    timestamp: String, // (should be iso8601 format)

    // User and context attributes.
    user_id: Option<u64>,
    // HTTP Request attributes.
    req_path: String,
    req_method: String,

    // Errors attributes.
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}
