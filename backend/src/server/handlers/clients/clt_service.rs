use common::ClientModel;
use diesel::prelude::*;
use diesel_async::{scoped_futures::ScopedFutureExt, AsyncPgConnection, RunQueryDsl};

use crate::db::{db_models::ClientDb, schema::clients};

use super::{clt_mapper::ClientMappers, clt_responses::ClientResponse};

pub struct ClientService;

impl ClientService {
    pub async fn get_client_by_id(
        conn: &mut AsyncPgConnection,
    ) -> Result<ClientModel, ClientServiceErr> {
        match clients::table
            .filter(clients::client_id.eq("C001"))
            .first::<ClientDb>(conn)
            .await
        {
            Ok(info) => Ok(ClientMappers::from_db(info)),
            Err(_) => Err(ClientServiceErr::DoesNotExist),
            _ => Err(ClientServiceErr::DbError),
        }
    }
}

pub enum ClientServiceErr {
    DoesNotExist,
    DbError,
    AlreadyExists,
}
