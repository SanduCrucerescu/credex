use axum::{
    routing::{get, post},
    Router,
};

use super::trx_controlers::TrxControler;

pub fn trx_routes() -> Router {
    Router::new()
        .route("/trx", get(TrxControler::get_trx))
        .route("/trx_post", post(TrxControler::post_trx))
}
