pub mod responses;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

#[derive(Serialize, Clone, PartialEq, Deserialize)]
pub struct ClientModel {
    pub name: String,
    pub email: String,
    pub password: String,
    pub date_of_birth: Datetime,
}

#[derive(Serialize, Debug, PartialEq, Clone, Deserialize)]
pub struct ClientLoginModel {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Debug, PartialEq, Clone, Deserialize)]
pub struct Account {
    pub acc_id: i32,
    pub client_id: String,
    pub balance: f64,
    pub acc_activation_date: Datetime,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}
