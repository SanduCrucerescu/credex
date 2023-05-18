pub mod responses;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Clone, PartialEq, Deserialize)]
pub struct ClientModel {
    pub client_id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub date_of_birth: SystemTime,
}

#[derive(Serialize, Debug, PartialEq, Clone, Deserialize)]
pub struct ClientCreateModel {
    pub name: String,
    pub email: String,
    pub password: String,
    pub date_of_birth: SystemTime,
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
    pub acc_activation_date: SystemTime,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}
