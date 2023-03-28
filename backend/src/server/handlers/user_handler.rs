use crate::db::db_models::{Client, Transaction};
use crate::db::db_utils::{AppState, DbActor};
use crate::db::messages::{GetClients, GetUserTransactions, PostUserTransactions};
use actix::Addr;
use actix_web::{
    get, post, web,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTransactionBody {
    pub sender_id: String,
    pub receiver_id: String,
    pub amount: f32,
}

#[get("/clients")]
pub async fn get_clients(state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();

    match db.send(GetClients).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No clients found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve clients"),
    }
}

#[get("/users/{id}/transactions")]
pub async fn get_user_transactions(state: Data<AppState>, path: Path<String>) -> impl Responder {
    let id = path.into_inner();
    let db = state.as_ref().db.clone();

    match db.send(GetUserTransactions { user_id: id }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No transactions found for user {id}"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve transactions"),
    }
}

#[post("/users/{id}/transactions")]
pub async fn post_user_transactions(
    state: Data<AppState>,
    path: Path<String>,
    body: Json<CreateTransactionBody>,
) -> impl Responder {
    let id = path.into_inner();
    let db = state.as_ref().db.clone();

    match db
        .send(PostUserTransactions {
            id: id.clone(),
            sender_id: id,
            receiver_id: body.receiver_id.clone(),
            amount: body.amount,
        })
        .await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to post transaction"),
    }
}
