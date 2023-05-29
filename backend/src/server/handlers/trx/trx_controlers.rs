use axum::{extract, Json};
use chrono::Utc;
use common::{Request, Response, TrxModel};

use crate::{error::Error, DBS};

pub struct TrxControler;

impl TrxControler {
    pub async fn post_trx(
        extract::Json(payload): extract::Json<TrxModel>,
    ) -> Result<Json<Response>, Error> {
        let _trx: TrxModel = DBS
            .create("transaction")
            .content(TrxModel {
                sender_acc_no: payload.sender_acc_no,
                receiver_acc_no: payload.receiver_acc_no,
                ammount: payload.ammount,
                trx_time: Utc::now(),
            })
            .await?;
        Ok(Json(Response {
            res: "Successfull".to_string(),
        }))
    }

    pub async fn get_trx(
        extract::Json(payload): extract::Json<Request>,
    ) -> Result<Json<TrxModel>, Error> {
        let querry = format!("SELECT * FROM transaction WHERE id = '{}';", payload.data);

        let mut select = DBS.query(querry).await?;
        let res: Option<TrxModel> = select.take(0)?;
        match res {
            Some(info) => Ok(Json(info)),
            None => Err(Error::Surreal(surrealdb::Error::Db(
                surrealdb::error::Db::IdInvalid {
                    value: payload.data,
                },
            ))),
        }
    }
}
