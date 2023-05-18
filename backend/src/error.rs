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

// impl axum::response::Response for Error {}

// impl From<Error> for String {
//     fn from(e: Error) -> String {
//         e.to_string()
//     }
// }

// impl From<base64::DecodeError> for Error {
//     fn from(_: base64::DecodeError) -> Self {
//         Error::InvalidAuth
//     }
// }

// impl From<std::string::FromUtf8Error> for Error {
//     fn from(_: std::string::FromUtf8Error) -> Self {
//         Error::InvalidAuth
//     }
// }

// impl From<jsonwebtoken::errors::Error> for Error {
//     fn from(_: jsonwebtoken::errors::Error) -> Self {
//         Error::InvalidAuth
//     }
// }

// impl From<surrealdb::error::Db> for Error {
//     fn from(value: surrealdb::error::Db) -> Self {
//         Error::Db(value.into())
//     }
// }
