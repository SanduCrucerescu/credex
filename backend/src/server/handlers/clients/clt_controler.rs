use axum::{
    extract::{self, Path},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use common::{
    responses::clt_responses::ClientCreationResponse, ClientCreateModel, ClientLoginModel,
    ClientModel,
};
use std::{sync::Arc, time::SystemTime};

use super::clt_service::ClientService;
use crate::{
    db::{
        db::{get_connection_from_pool, AppState},
        db_models::ClientDb,
    },
    error::Error,
    DBS,
};
use common::responses::clt_responses::{ClientLoginResponse, ClientResponse};

pub struct ClientControler;

impl ClientControler {
    // pub async fn get_client(
    //     Extension(state): Extension<Arc<AppState>>,
    //     Path(id): Path<String>,
    // ) -> Result<ClientResponse, ClientControlerErr> {
    //     let mut conn = get_connection_from_pool(&state.db_pool)
    //         .await
    //         .map_err(|_| ClientControlerErr::ConnectionErr)?;

    //     match ClientService::get_client_by_id(conn.as_mut(), &id).await {
    //         Ok(info) => Ok(ClientResponse {
    //             client_id: info.client_id,
    //             name: info.name,
    //             email: info.email,
    //             password: info.password,
    //             date_of_birth: info.date_of_birth,
    //         }),
    //         Err(e) => match e {
    //             super::clt_service::ClientServiceErr::DoesNotExist => {
    //                 Err(ClientControlerErr::InvalidClient)
    //             }
    //             _ => Err(ClientControlerErr::InternalError),
    //         },
    //     }
    // }

    // pub async fn post_login(
    //     Extension(state): Extension<Arc<AppState>>,
    //     extract::Json(payload): extract::Json<ClientLoginModel>,
    // ) -> Result<ClientLoginResponse, ClientControlerErr> {
    //     let mut conn = get_connection_from_pool(&state.db_pool)
    //         .await
    //         .map_err(|_| ClientControlerErr::ConnectionErr)?;

    //     match ClientService::post_client_login(conn.as_mut(), &payload).await {
    //         Ok(info) => {
    //             let mut x = info;
    //             x.status = Some("200".to_string());
    //             x.msg = Some("Successfull".to_string());
    //             Ok(x)
    //         }
    //         Err(_) => Err(ClientControlerErr::InvalidClient),
    //     }
    // }

    pub async fn post_client(
        // Extension(state): Extension<Arc<AppState>>,
        extract::Json(payload): extract::Json<ClientCreateModel>,
    ) -> Result<Json<ClientModel>, Error> {
        // let mut conn = get_connection_from_pool(&state.db_pool)
        //     .await
        //     .map_err(|_| ClientControlerErr::ConnectionErr)?;

        // match ClientService::post_new_client(payload).await {
        //     Ok(_) => Ok(ClientCreationResponse {
        //         status: "200".to_string(),
        //         msg: "Client creaded successfuly".to_string(),
        //     }),
        //     Err(_) => Err(ClientControlerErr::InternalError),
        // }
        // ClientService::post_new_client(payload).await;
        // Ok(ClientCreationResponse {
        //     status: "200".to_string(),
        //     msg: "Client creaded successfuly".to_string(),
        // })

        let t = DBS
            .create(("person", "fdgtrterger"))
            .content(ClientModel {
                client_id: "fghytgrtf".to_string(),
                name: "ttttttttt".to_string(),
                email: "restds".to_string(),
                password: "ereed".to_string(),
                date_of_birth: SystemTime::now(),
            })
            .await?;
        // println!("{:?}", t);
        Ok(Json(t))
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
            Self::InternalError => "An error occurred while processing the request",
            Self::InvalidClient => "Client does not exist",
            Self::ConnectionErr => "Connection error",
        }
    }
}

impl IntoResponse for ClientControlerErr {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, self.error_message()).into_response()
    }
}
