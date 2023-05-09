// use crate::db::db_utils::AppState;
// use crate::db::messages::{
//     GetClient, GetClients, GetUserTransactions, PostLogin, PostUserTransactions,
// };
// use actix_web::{
//     get, post,
//     web::{Data, Json, Path},
//     HttpResponse, Responder,
// };
// use chrono::NaiveDateTime;
// use common::UserLoginResponse;
// use serde::Deserialize;

// #[derive(Deserialize)]
// pub struct CreateTransactionBody {
//     pub sender_id: String,
//     pub receiver_id: String,
//     pub amount: f32,
// }

// #[derive(Deserialize, Clone, Debug)]
// pub struct CreateLoginBody {
//     pub email: String,
//     pub password: String,
// }

// #[get("/clients")]
// pub async fn get_clients(state: Data<AppState>) -> impl Responder {
//     let db = state.as_ref().db.clone();

//     match db.send(GetClients).await {
//         Ok(Ok(info)) => HttpResponse::Ok().json(info),
//         Ok(Err(_)) => HttpResponse::NotFound().json("No clients found"),
//         _ => HttpResponse::InternalServerError().json("Unable to retrieve clients"),
//     }
// }

// #[get("/client/{id}")]
// pub async fn get_client(state: Data<AppState>, path: Path<String>) -> impl Responder {
//     let id = path.into_inner();
//     let db = state.as_ref().db.clone();

//     match db.send(GetClient { client_id: id }).await {
//         Ok(Ok(info)) => HttpResponse::Ok().json(info),
//         Ok(Err(_)) => HttpResponse::NotFound().json("No client found"),
//         _ => HttpResponse::InternalServerError().json("Unable to retrieve client"),
//     }
// }

// #[get("/users/{id}/transactions")]
// pub async fn get_user_transactions(state: Data<AppState>, path: Path<String>) -> impl Responder {
//     let id = path.into_inner();
//     let db = state.as_ref().db.clone();

//     match db.send(GetUserTransactions { user_id: id }).await {
//         Ok(Ok(info)) => HttpResponse::Ok().json(info),
//         Ok(Err(_)) => HttpResponse::NotFound().json("No transactions found for user {id}"),
//         _ => HttpResponse::InternalServerError().json("Unable to retrieve transactions"),
//     }
// }

// // change body to the extractor
// #[post("/login")]
// pub async fn post_login(state: Data<AppState>, body: Json<CreateLoginBody>) -> impl Responder {
//     let db = state.as_ref().db.clone();
//     match db
//         .send(PostLogin {
//             email: body.email.clone(),
//             password: body.password.clone(),
//         })
//         .await
//     {
//         Ok(Ok(info)) => {
//             println!("here");
//             HttpResponse::Ok().json(UserLoginResponse {
//                 status: "200".to_string(),
//                 message: "Successfull".to_string(),
//                 user_id: Some(info.client_id),
//             })
//         }
//         Ok(Err(_)) => HttpResponse::NotFound().json(UserLoginResponse {
//             status: "404".to_string(),
//             message: "User not found".to_string(),
//             user_id: None,
//         }),
//         _ => HttpResponse::InternalServerError().json("Failed to post"),
//     }
// }

// #[post("/users/{id}/transactions")]
// pub async fn post_user_transactions(
//     state: Data<AppState>,
//     path: Path<String>,
//     body: Json<CreateTransactionBody>,
// ) -> impl Responder {
//     let id = path.into_inner();
//     let db = state.as_ref().db.clone();

//     match db
//         .send(PostUserTransactions {
//             id: id.clone(),
//             sender_id: id,
//             receiver_id: body.receiver_id.clone(),
//             amount: body.amount,
//         })
//         .await
//     {
//         Ok(Ok(info)) => HttpResponse::Ok().json(info),
//         _ => HttpResponse::InternalServerError().json("Failed to post transaction"),
//     }
// }
