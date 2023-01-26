use crate::db::db_models::{Transactions, User};
use crate::db::db_utils::{AppState, DbActor};
use crate::db::messages::GetUsers;
use actix::Addr;
use actix_web::{
    get, post, web,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};

#[get("/users")]
pub async fn get_users(state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();

    match db.send(GetUsers).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No users found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
    }
}

// pub fn get_user_transactions(path: Path<String>) -> impl Responder {
//     let id = path.into_inner();
//     format!("GET /users/{id}/transactions")
// }

// pub fn post_user_transactions(path: Path<String>, body: Json<Transactions>) -> impl Responder {
//     let id = path.into_inner();
//     format!("POST /users/{id}/transactions")
// }
