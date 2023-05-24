use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[cfg(feature = "back")]
use surrealdb::sql::Thing;

#[derive(Serialize, Clone, Debug, PartialEq, Deserialize)]
pub struct ClientModel {
    pub name: String,
    pub email: String,
    pub password: String,
    #[serde(with = "ts_seconds")]
    pub date_of_birth: DateTime<Utc>,
}

#[derive(Serialize, Clone, Debug, PartialEq, Deserialize)]
pub struct ClientInsertModel {
    pub name: String,
    pub email: String,
    pub password: String,
    #[serde(with = "ts_seconds")]
    pub date_of_birth: DateTime<Utc>,
    pub account: AccountModel,
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
pub struct AccountModel {
    pub balance: f64,
    #[serde(with = "ts_seconds")]
    pub acc_activation_date: DateTime<Utc>,
}
