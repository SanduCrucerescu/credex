use crate::db::schema::{clients, transactions};
use chrono::NaiveDateTime;
use diesel::Insertable;
use serde::Serialize;

#[derive(Insertable, Clone, Serialize)]
#[diesel(table_name = transactions)]
pub struct NewTransaction {
    pub id: String,
    pub sender_id: String,
    pub receiver_id: String,
    pub amount: f32,
    pub withdrawal_time: NaiveDateTime,
}

#[derive(Insertable, Clone, Serialize)]
#[diesel(table_name = clients)]
pub struct NewClient {
    pub client_id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub date_of_birth: NaiveDateTime,
}

#[derive(Insertable, Clone, Serialize)]
#[diesel(table_name = clients)]
pub struct NewLogin {
    pub client_id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub date_of_birth: NaiveDateTime,
}
