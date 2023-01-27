use crate::db::db_models::{Transactions, User};
use crate::db::db_utils::DbActor;
use crate::db::messages::{GetUserTransactions, GetUsers, PostUserTransactions};
use crate::db::schema::transactions::dsl::*;
use crate::db::schema::users::{dsl::*, id as transaction_id};
use actix::Handler;
use diesel::{self, prelude::*};

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
    type Result = QueryResult<Vec<Transactions>>;

    fn handle(&mut self, msg: GetUserTransactions, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Fetch user transactions: Unable to establish connection");

        transactions
            .filter(sender_id.eq(msg.user_id))
            .get_results::<Transactions>(&mut conn)
    }
}

impl Handler<PostUserTransactions> for DbActor {
    type Result = QueryResult<Transactions>;

    fn handle(&mut self, msg: PostUserTransactions, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Fetch user transactions: Unable to establish connection");

        diesel::insert_into(transactions)
            .values(&msg.body)
            .get_result::<Transactions>(&mut conn)
    }
}
