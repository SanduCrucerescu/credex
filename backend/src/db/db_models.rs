#![allow(unused)]
#![allow(clippy::all)]

use std::time::SystemTime;

use super::schema::{accounts, clients};
use chrono::NaiveDateTime;
use chrono::{offset::Utc, serde::ts_seconds, DateTime};
use diesel::{Identifiable, Insertable, Queryable};
use serde::Deserialize;
use serde::Serialize;

#[derive(Queryable, Debug, Serialize, Identifiable, Insertable, Deserialize)]
#[diesel(primary_key(client_id), table_name = clients)]
pub struct ClientDb {
    pub client_id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    #[serde(skip_serializing)]
    pub date_of_birth: SystemTime,
}

#[derive(Queryable, Debug, Serialize, Identifiable, Insertable)]
#[diesel(primary_key(acc_id), table_name = accounts)]
pub struct AccountDb {
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
