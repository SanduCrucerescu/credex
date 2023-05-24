pub mod responses;
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

#[derive(Serialize, Clone, Debug, PartialEq, Deserialize)]
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

#[derive(Serialize, Debug, Deserialize)]
pub struct ClientLoginResponse {
    pub id: Thing,
}

#[derive(Serialize, Debug, PartialEq, Clone, Deserialize)]
pub struct Account {
    pub acc_id: i32,
    pub client_id: String,
    pub balance: f64,
    pub acc_activation_date: Datetime,
}
