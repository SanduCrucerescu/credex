use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, PartialEq, Clone, Deserialize)]
pub struct ClientModel {
    pub client_id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    // #[serde(skip_serializing)]
    // #[serde(skip_deserializing)]
    // pub date_of_birth: NaiveDateTime,
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
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub acc_activation_date: NaiveDateTime,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

// #[derive(Serialize, Debug, Deserialize)]
// pub struct ClientLoginResponse {
//     pub status: String,
//     pub message: String,
//     // pub user_id: Option<String>,
// }
#[derive(Serialize, Debug, Deserialize)]
pub struct ClientLoginResponse {
    pub status: String,
    pub msg: String,
}
