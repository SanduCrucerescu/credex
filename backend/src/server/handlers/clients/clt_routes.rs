use axum::{routing::get, Router};

use super::clt_controler::ClientControler;

pub fn clt_routes() -> Router {
    Router::new().route("/:client_id", get(ClientControler::get_client))
}
