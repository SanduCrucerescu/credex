use crate::db::db_models::{Transaction, User};
use crate::db::db_utils::DbActor;
use crate::db::messages::{GetUserTransactions, GetUsers, PostUserTransactions};
use crate::db::schema::transactions::{dsl::*, id as transaction_id};
use crate::db::schema::users::dsl::*;
use actix::Handler;
use diesel::{self, prelude::*};

use super::insertables::NewTransaction;

impl Handler<GetUsers> for DbActor {
    type Result = QueryResult<Vec<User>>;

    fn handle(&mut self, _msg: GetUsers, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Fetch User: Unable to establish connection");

        users.get_results::<User>(&mut conn)
    }
}

impl Handler<GetUserTransactions> for DbActor {
    type Result = QueryResult<Vec<Transaction>>;

    fn handle(&mut self, msg: GetUserTransactions, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Fetch user transactions: Unable to establish connection");

        transactions
            .filter(sender_id.eq(msg.user_id))
            .get_results::<Transaction>(&mut conn)
    }
}

impl Handler<PostUserTransactions> for DbActor {
    type Result = QueryResult<Transaction>;

    fn handle(&mut self, msg: PostUserTransactions, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Fetch user transactions: Unable to establish connection");

        let new_transaction = NewTransaction {
            id: msg.id,
            sender_id: msg.sender_id,
            receiver_id: msg.receiver_id,
            amount: msg.amount,
        };
        diesel::insert_into(transactions)
            .values(new_transaction)
            .returning((transaction_id, sender_id, receiver_id, amount))
            .get_result::<Transaction>(&mut conn)
    }
}
