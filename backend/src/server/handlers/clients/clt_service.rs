use std::time::{self, SystemTime};

use axum::Json;
use chrono::{DateTime, NaiveDateTime};
use common::{
    responses::clt_responses::ClientLoginResponse, ClientCreateModel, ClientLoginModel, ClientModel,
};
use diesel::{dsl, insert_into, prelude::*};
use diesel_async::{scoped_futures::ScopedFutureExt, AsyncPgConnection, RunQueryDsl};
use serde::Deserialize;
use surrealdb::sql::Thing;
use uuid::Uuid;

use crate::{
    db::{
        db_models::{AccountDb, ClientDb},
        schema::{accounts, clients},
    },
    error::Error,
};
#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}
use crate::DBS;

use super::clt_mapper::ClientMappers;

pub struct ClientService;

impl ClientService {
    // pub async fn get_client_by_id(
    //     conn: &mut AsyncPgConnection,
    //     id: &str,
    // ) -> Result<ClientModel, ClientServiceErr> {
    //     let client = clients::table
    //         .filter(clients::client_id.eq(id))
    //         .get_result::<ClientDb>(conn)
    //         .await
    //         .map_err(ClientServiceErr::from)?;
    //     Ok(ClientMappers::client_db(client))
    // }

    // pub async fn post_client_login(
    //     conn: &mut AsyncPgConnection,
    //     login_details: &ClientLoginModel,
    // ) -> Result<ClientLoginResponse, ClientServiceErr> {
    //     let client_login = clients::table
    //         .select(clients::client_id)
    //         .filter(clients::email.eq(&login_details.email))
    //         .filter(clients::password.eq(&login_details.password))
    //         .get_result::<String>(conn)
    //         .await
    //         .map_err(ClientServiceErr::from)?;

    //     Ok(ClientMappers::login_db(client_login))
    // }

    pub async fn post_new_client(
        // conn: &mut AsyncPgConnection,
        new_client: ClientCreateModel,
    ) -> Result<ClientDb, Error> {
        let id = Uuid::new_v4().to_string();
        // let new_client = conn
        //     .build_transaction()
        //     .read_write()
        //     .run(|conn| {
        //         async move {
        //             let client = insert_into(clients::table)
        //                 .values((
        //                     clients::client_id.eq(&id),
        //                     clients::name.eq(new_client.name),
        //                     clients::email.eq(new_client.email),
        //                     clients::password.eq(new_client.password),
        //                     clients::date_of_birth.eq(new_client.date_of_birth),
        //                 ))
        //                 .get_result::<ClientDb>(conn)
        //                 .await?;

        //             let acc = insert_into(accounts::table)
        //                 .values((
        //                     accounts::acc_id.eq(4342423),
        //                     accounts::client_id.eq(&id),
        //                     accounts::balance.eq(0.0),
        //                     accounts::acc_activation_date.eq(dsl::now),
        //                 ))
        //                 .get_result::<AccountDb>(conn)
        //                 .await?;

        //             Ok(client)
        //         }
        //         .scope_boxed()
        //     })
        //     .await?;
        // .map_err(|err: diesel::result::Error| ClientServiceErr::from(err))?;
        let t = DBS
            .create(("person", "dfdfdsfff"))
            .content(ClientDb {
                client_id: id,
                name: "ttttttttt".to_string(),
                email: "restds".to_string(),
                password: "ereed".to_string(),
                date_of_birth: SystemTime::now(),
            })
            .await?;
        println!("{:?}", t);
        Ok(t)
    }
}

// #[derive(Error, Debug)]
// pub enum ClientServiceErr {
//     DoesNotExist,
//     DbError,
//     AlreadyExists,
// }

// impl From<diesel::result::Error> for ClientServiceErr {
//     fn from(diesel_err: diesel::result::Error) -> Self {
//         match diesel_err {
//             diesel::result::Error::DatabaseError(err, _) => match err {
//                 diesel::result::DatabaseErrorKind::UniqueViolation => Self::AlreadyExists,
//                 _ => Self::DbError,
//             },
//             diesel::result::Error::NotFound => Self::DoesNotExist,
//             _ => Self::DbError,
//         }
//     }
// }
