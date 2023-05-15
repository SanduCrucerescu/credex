use axum::Json;
use common::{
    responses::clt_responses::ClientLoginResponse, ClientCreateModel, ClientLoginModel, ClientModel,
};
use diesel::{insert_into, prelude::*};
use diesel_async::{scoped_futures::ScopedFutureExt, AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::db::{db_models::ClientDb, schema::clients::dsl::*};

use super::clt_mapper::ClientMappers;

pub struct ClientService;

impl ClientService {
    pub async fn get_client_by_id(
        conn: &mut AsyncPgConnection,
        id: &str,
    ) -> Result<ClientModel, ClientServiceErr> {
        let client = clients
            .filter(client_id.eq(id))
            .get_result::<ClientDb>(conn)
            .await
            .map_err(ClientServiceErr::from)?;
        Ok(ClientMappers::client_db(client))
    }

    pub async fn post_client_login(
        conn: &mut AsyncPgConnection,
        login_details: &ClientLoginModel,
    ) -> Result<ClientLoginResponse, ClientServiceErr> {
        let client_login = clients
            .select(client_id)
            .filter(email.eq(&login_details.email))
            .filter(password.eq(&login_details.password))
            .get_result::<String>(conn)
            .await
            .map_err(ClientServiceErr::from)?;

        Ok(ClientMappers::login_db(client_login))
    }

    pub async fn post_new_client(
        conn: &mut AsyncPgConnection,
        new_client: ClientCreateModel,
    ) -> Result<ClientModel, ClientServiceErr> {
        let id = Uuid::new_v4().to_string();
        let new_client = conn
            .build_transaction()
            .read_write()
            .run(|conn| {
                async move {
                    let client = insert_into(clients)
                        .values((
                            client_id.eq(&id),
                            name.eq(new_client.name),
                            email.eq(new_client.email),
                            password.eq(new_client.password),
                            date_of_birth.eq(new_client.date_of_birth),
                        ))
                        .get_result::<ClientDb>(conn)
                        .await?;
                    Ok(client)
                }
                .scope_boxed()
            })
            .await
            .map_err(|err: diesel::result::Error| ClientServiceErr::from(err))?;
        Ok(ClientMappers::client_db(new_client))
    }
}

pub enum ClientServiceErr {
    DoesNotExist,
    DbError,
    AlreadyExists,
}

impl From<diesel::result::Error> for ClientServiceErr {
    fn from(diesel_err: diesel::result::Error) -> Self {
        match diesel_err {
            diesel::result::Error::DatabaseError(err, _) => match err {
                diesel::result::DatabaseErrorKind::UniqueViolation => Self::AlreadyExists,
                _ => Self::DbError,
            },
            diesel::result::Error::NotFound => Self::DoesNotExist,
            _ => Self::DbError,
        }
    }
}
