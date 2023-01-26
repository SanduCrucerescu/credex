use actix::Message;
use db_models::{Transactions, User};
use diesel::QueryResult;

use super::db_models;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<User>>")]
pub struct GetUsers;
