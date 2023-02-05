use actix::Message;
use db_models::{Transaction, User};
use diesel::QueryResult;

use super::db_models;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<User>>")]
pub struct GetUsers;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Transaction>>")]
pub struct GetUserTransactions {
    pub user_id: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Transaction>")]
pub struct PostUserTransactions {
    pub id: String,
    pub sender_id: String,
    pub receiver_id: String,
    pub amount: f32,
}
