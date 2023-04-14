#![allow(unused)]
#![allow(clippy::all)]

use chrono::NaiveDateTime;
use chrono::{offset::Utc, serde::ts_seconds, DateTime};
use diesel::Queryable;
use serde::Deserialize;
use serde::Serialize;

#[derive(Queryable, Debug, Serialize)]
pub struct Client {
    pub client_id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    #[serde(skip_serializing)]
    pub date_of_birth: NaiveDateTime,
}

#[derive(Queryable, Debug, Serialize)]
pub struct Account {
    pub acc_id: i32,
    pub client_id: String,
    pub balance: f32,
    #[serde(skip_serializing)]
    pub acc_activation_date: NaiveDateTime,
}

#[derive(Queryable, Debug, Serialize)]
pub struct Transaction {
    pub id: String,
    pub sender_id: String,
    pub receiver_id: String,
    pub amount: f32,
    #[serde(skip_serializing)]
    pub withdrawal_time: NaiveDateTime,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}
