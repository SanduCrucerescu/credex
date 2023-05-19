use axum::{
    extract::{self, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use common::ClientModel;
use surrealdb::sql::Datetime;

use crate::{error::Error, DBS};

pub struct ClientControler;

impl ClientControler {
    pub async fn get_client(Path(id): Path<String>) -> Result<Json<ClientModel>, Error> {
        let client: Option<ClientModel> = DBS.select(("client", id.clone())).await?;
        match client {
            Some(info) => Ok(Json(info)),
            None => Err(Error::Surreal(surrealdb::Error::Db(
                surrealdb::error::Db::IdInvalid { value: id.clone() },
            ))),
        }
    }

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
        extract::Json(payload): extract::Json<ClientModel>,
    ) -> Result<Json<ClientModel>, Error> {
        let client = DBS
            .create("client")
            .content(ClientModel {
                name: payload.name.to_string(),
                email: payload.email.to_string(),
                password: payload.password.to_string(),
                date_of_birth: Datetime::from(payload.date_of_birth),
            })
            .await?;
        Ok(Json(client))
    }
}
