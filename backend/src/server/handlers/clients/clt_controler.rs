use axum::{
    extract::{self, Path},
    Json,
};
use chrono::Utc;
use common::{AccountModel, ClientInsertModel, ClientLoginModel, ClientLoginResponse, ClientModel};

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
        let acc = AccountModel {
            balance: 0.0,
            acc_activation_date: Utc::now(),
        };
        let client = DBS
            .create("client")
            .content(ClientInsertModel {
                name: payload.name.to_string(),
                email: payload.email.to_string(),
                password: payload.password.to_string(),
                date_of_birth: payload.date_of_birth,
                account: acc,
            })
            .await?;
        Ok(Json(client))
    }
}
