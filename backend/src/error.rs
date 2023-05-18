use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::Error as JsonError;
use surrealdb::Error as SurrealError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("The request body contains invalid data")]
    Request,

    #[error("The specified media type is unsupported")]
    InvalidType,

    #[error("There was an error serializing JSON: {0}")]
    Json(#[from] JsonError),

    #[error("There was a problem with the database")]
    Db,

    #[error("Could not open the requested file: {0}")]
    Io(#[from] std::io::Error),

    #[error("There was a problem eith authentication.")]
    InvalidAuth,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self.to_string())).into_response()
    }
}

impl From<surrealdb::Error> for Error {
    fn from(value: surrealdb::Error) -> Self {
        eprintln!("{value}");
        Self::Db
    }
}
