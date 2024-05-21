use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

// NOTE: ___________________________ VARIABLES ___________________________
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize)]
pub enum Error {
    LoginFail,
    // Auth errors.
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,

    // Model errors
    TicketDeleteFailIdNotFound { id: u64 },
}

// HACK: _______________________ Error Boilerplate _______________________
impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// ______________________________________________________________________
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}
