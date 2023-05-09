use common::ClientModel;

use crate::db::db_models::ClientDb;

pub struct ClientMappers;

impl ClientMappers {
    pub fn from_db(client_db: ClientDb) -> ClientModel {
        ClientModel {
            client_id: client_db.client_id,
            name: client_db.name,
            email: client_db.email,
            password: client_db.password,
            // date_of_birth: client_db.date_of_birth,
        }
    }
}
