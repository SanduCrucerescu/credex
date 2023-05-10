use common::{ClientLoginModel, ClientModel};

use crate::db::db_models::{ClientDb, LoginDb};

pub struct ClientMappers;

impl ClientMappers {
    pub fn client_db(client_db: ClientDb) -> ClientModel {
        ClientModel {
            client_id: client_db.client_id,
            name: client_db.name,
            email: client_db.email,
            password: client_db.password,
            // date_of_birth: client_db.date_of_birth,
        }
    }
    pub fn login_db(response: LoginDb) -> ClientLoginModel {
        ClientLoginModel {
            email: response.email,
            password: response.password,
        }
    }
}
