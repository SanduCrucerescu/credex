use axum::{response::IntoResponse, Json};
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct ClientResponse {
    pub client_id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    // pub date_of_birth: NaiveDateTime,
}

impl IntoResponse for ClientResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
#[derive(Serialize)]
pub struct ClientLoginResponse {
    pub status: String,
    pub msg: String,
}
