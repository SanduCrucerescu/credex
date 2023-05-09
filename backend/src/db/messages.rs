// use actix::Message;
// use chrono::NaiveDateTime;
// use db_models::{Client, Login, Transaction};
// use diesel::QueryResult;

// use super::db_models;

// #[derive(Message)]
// #[rtype(result = "QueryResult<Vec<Client>>")]
// pub struct GetClients;

// #[derive(Message)]
// #[rtype(result = "QueryResult<Client>")]
// pub struct GetClient {
//     pub client_id: String,
// }

// #[derive(Message)]
// #[rtype(result = "QueryResult<Vec<Transaction>>")]
// pub struct GetUserTransactions {
//     pub user_id: String,
// }

// #[derive(Message)]
// #[rtype(result = "QueryResult<Transaction>")]
// pub struct PostUserTransactions {
//     pub id: String,
//     pub sender_id: String,
//     pub receiver_id: String,
//     pub amount: f32,
// }

// #[derive(Message)]
// #[rtype(result = "QueryResult<Login>")]
// pub struct PostLogin {
//     pub email: String,
//     pub password: String,
// }
