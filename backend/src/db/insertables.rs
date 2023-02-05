use crate::db::schema::transactions;
use diesel::Insertable;
use serde::Serialize;

#[derive(Insertable, Clone, Serialize)]
#[diesel(table_name = transactions)]
pub struct NewTransaction {
    pub id: String,
    pub sender_id: String,
    pub receiver_id: String,
    pub amount: f32,
}
