use crate::db::db_models::{Client, Transaction};
use crate::db::db_utils::DbActor;
use crate::db::messages::{GetClients, GetUserTransactions, PostUserTransactions};
use crate::db::schema::clients::dsl::*;
use crate::db::schema::transactions::{dsl::*, id as transaction_id};
use actix::Handler;
use diesel::{self, prelude::*};

use super::insertables::NewTransaction;

impl Handler<GetClients> for DbActor {
    type Result = QueryResult<Vec<Client>>;

    fn handle(&mut self, _msg: GetClients, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Fetch User: Unable to establish connection");

        clients.get_results::<Client>(&mut conn)
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
