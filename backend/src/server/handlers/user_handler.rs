use crate::db::db_models::{Transaction, User};
use crate::db::db_utils::{AppState, DbActor};
use crate::db::messages::{GetUserTransactions, GetUsers, PostUserTransactions};
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

#[get("/users")]
pub async fn get_users(state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();

    match db.send(GetUsers).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No users found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
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
