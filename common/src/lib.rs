use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, PartialEq, Clone, Deserialize)]
pub struct Client {
    pub client_id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub balance: f64,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub date_of_birth: NaiveDateTime,
}
#[derive(Serialize, Debug, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}
