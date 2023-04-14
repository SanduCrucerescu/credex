use crate::db::db_models::{Client, Login, Transaction};
use crate::db::db_utils::DbActor;
use crate::db::messages::{
    GetClient, GetClients, GetUserTransactions, PostLogin, PostUserTransactions,
};
use crate::db::schema::clients::dsl::*;
use crate::db::schema::transactions::{dsl::*, id as transaction_id};
use actix::Handler;
use diesel::{self, prelude::*};

use super::insertables::{NewClient, NewLogin, NewTransaction};

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

impl Handler<GetClient> for DbActor {
    type Result = QueryResult<Client>;

    fn handle(&mut self, msg: GetClient, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Fetch User: Unable to establish connection");

        clients
            .filter(client_id.eq(msg.client_id))
            .get_result::<Client>(&mut conn)
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
            withdrawal_time: chrono::Utc::now().naive_utc(),
        };
        diesel::insert_into(transactions)
            .values(new_transaction)
            .returning((
                transaction_id,
                sender_id,
                receiver_id,
                amount,
                withdrawal_time,
            ))
            .get_result::<Transaction>(&mut conn)
    }
}

impl Handler<PostLogin> for DbActor {
    type Result = QueryResult<Login>;

    fn handle(&mut self, msg: PostLogin, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Login: Unable to establish connection");

        clients
            .select((email, password))
            .filter(email.eq(msg.email))
            .filter(password.eq(msg.password))
            .get_result::<Login>(&mut conn)
    }
}
