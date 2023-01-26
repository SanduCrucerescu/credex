use crate::db::db_models::{Transactions, User};
use crate::db::db_utils::DbActor;
use crate::db::messages::GetUsers;
use crate::db::schema::users::dsl::*;
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
