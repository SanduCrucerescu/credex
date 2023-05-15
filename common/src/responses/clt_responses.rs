#[cfg(feature = "back")]
use axum::{response::IntoResponse, Json};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ClientResponse {
    pub client_id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    #[serde(skip_serializing)]
    pub date_of_birth: NaiveDateTime,
}

#[cfg(feature = "back")]
impl IntoResponse for ClientResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
#[derive(Serialize, Deserialize)]
pub struct ClientLoginResponse {
    pub status: Option<String>,
    pub msg: Option<String>,
    pub client_id: Option<String>,
}

#[cfg(feature = "back")]
impl IntoResponse for ClientLoginResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

#[derive(Serialize, Deserialize)]
pub struct ClientCreationResponse {
    pub status: String,
    pub msg: String,
}

#[cfg(feature = "back")]
impl IntoResponse for ClientCreationResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
