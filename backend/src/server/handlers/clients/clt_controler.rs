use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use std::sync::Arc;

use crate::db::db::{get_connection_from_pool, AppState};

use super::{clt_responses::ClientResponse, clt_service::ClientService};

pub struct ClientControler;

impl ClientControler {
    pub async fn get_client(
        Extension(state): Extension<Arc<AppState>>,
    ) -> Result<ClientResponse, ClientControlerErr> {
        let mut conn = get_connection_from_pool(&state.db_pool)
            .await
            .map_err(|_| ClientControlerErr::InternalError)?;

        match ClientService::get_client_by_id(conn.as_mut()).await {
            Ok(info) => Ok(ClientResponse {
                client_id: info.client_id,
                name: info.name,
                email: info.email,
                password: info.password,
            }),
            _ => Err(ClientControlerErr::InternalError),
        }
    }
}

pub enum ClientControlerErr {
    InternalError,
    InvalidClient,
    ConnectionErr,
}

impl ClientControlerErr {
    pub fn error_message(&self) -> &'static str {
        match self {
            Self::InternalError => "An error occurred while processing your request. Please try again later.",
            Self::InvalidClient => "An account is already associated with that email. Please login or use a different email.",
            Self::ConnectionErr => "Connection error"
        }
    }
}

impl IntoResponse for ClientControlerErr {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, self.error_message()).into_response()
    }
}
