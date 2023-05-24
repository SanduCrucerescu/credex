use axum::{
    routing::{get, post},
    Router,
};

use super::clt_controler::ClientControler;

pub fn clt_routes() -> Router {
    Router::new()
        .route("/:client_id", get(ClientControler::get_client))
        .route("/login", post(ClientControler::get_login))
        .route("/register", post(ClientControler::post_client))
}
