use deadpool::managed::{self, Object};
use diesel_async::pg::AsyncPgConnection;
use diesel_async::pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager};

use std::sync::Arc;

pub struct AppState {
    pub db_pool: Arc<Pool<AsyncPgConnection>>,
}

pub fn build_connection(db_url: &str) -> Pool<AsyncPgConnection> {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    Pool::builder(manager)
        .max_size(10)
        .build()
        .expect("Cannot create pool.")
}

pub async fn get_connection(
    db_pool: &Arc<Pool<AsyncPgConnection>>,
) -> Result<Object<AsyncPgConnection>, AsyncPoolError> {
    let managed_conn = db_pool
        .clone()
        .as_ref()
        .get()
        .await
        .map_err(|_| AsyncPoolError::Timeout)?;

    Ok(managed_conn)
}

pub enum AsyncPoolError {
    Timeout,
}
