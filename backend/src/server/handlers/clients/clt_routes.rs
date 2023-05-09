use axum::{routing::get, Router};

use super::clt_controler::ClientControler;

pub fn clt_routes() -> Router {
    Router::new().route("/", get(ClientControler::get_client))
}
