use common::{responses::clt_responses::ClientLoginResponse, ClientLoginModel, ClientModel};

use crate::{db::db_models::ClientDb, utils::macros::map};

pub struct ClientMappers;

impl ClientMappers {
    pub fn client_db(client_db: ClientDb) -> ClientModel {
        ClientModel {
            client_id: client_db.client_id,
            name: client_db.name,
            email: client_db.email,
            password: client_db.password,
            date_of_birth: client_db.date_of_birth,
        }
    }
    pub fn login_db(response: String) -> ClientLoginResponse {
        ClientLoginResponse {
            status: None,
            msg: None,
            client_id: Some(response),
        }
    }
}
