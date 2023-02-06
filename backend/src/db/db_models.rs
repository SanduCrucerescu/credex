#![allow(unused)]
#![allow(clippy::all)]

use chrono::{offset::Utc, serde::ts_seconds, DateTime};
use diesel::Queryable;
use serde::Deserialize;
use serde::Serialize;

#[derive(Queryable, Debug, Serialize)]
pub struct User {
    pub user_id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    // #[serde(with = "ts_seconds")]
    #[serde(skip_serializing)]
    pub created_at: DateTime<Utc>,
    // #[serde(with = "ts_seconds")]
    #[serde(skip_serializing)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, Debug, Serialize)]
pub struct Transaction {
    pub id: String,
    pub sender_id: String,
    pub receiver_id: String,
    pub amount: f32,
    // #[serde(skip_serializing)]
    // pub created_at: DateTime<Utc>,
    // #[serde(skip_serializing)]
    // pub received_at: DateTime<Utc>,
}
