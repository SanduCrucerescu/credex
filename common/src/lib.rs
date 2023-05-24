use chrono::{Date, DateTime, Utc};
use serde::{Deserialize, Serialize};
#[cfg(feature = "back")]
use surrealdb::sql::{Datetime, Thing};

#[derive(Serialize, Clone, Debug, PartialEq, Deserialize)]
pub struct ClientModel {
    pub name: String,
    pub email: String,
    pub password: String,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub date_of_birth: DateTime<Utc>,
}

#[derive(Serialize, Debug, PartialEq, Clone, Deserialize)]
pub struct ClientLoginModel {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Debug, Deserialize)]
#[cfg(feature = "back")]
pub struct ClientLoginResponse {
    pub id: Thing,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct ClientLoginRespons {
    pub id: String,
}

#[derive(Serialize, Debug, PartialEq, Clone, Deserialize)]
pub struct Account {
    pub acc_id: i32,
    pub client_id: String,
    pub balance: f64,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub acc_activation_date: DateTime<Utc>,
}
