use axum::{
    extract::{self, Path},
    Json,
};
use common::{ClientLoginModel, ClientLoginResponse, ClientModel};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{thing, Datetime, Thing};

use crate::{error::Error, DBS};
#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct User {
    name: String,
    company: String,
}
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

    pub async fn get_login(
        extract::Json(payload): extract::Json<ClientLoginModel>,
    ) -> Result<Json<ClientLoginResponse>, Error> {
        let querry = format!(
            "SELECT id from client WHERE email = '{}' AND password = '{}';",
            payload.email, payload.password,
        );
        let mut login = DBS.query(querry).await?;
        let res: Option<ClientLoginResponse> = login.take(0)?;
        match res {
            Some(info) => Ok(Json(info)),
            None => Err(Error::Surreal(surrealdb::Error::Db(
                surrealdb::error::Db::ScNotFound {
                    value: payload.email,
                },
            ))),
        }
    }

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
