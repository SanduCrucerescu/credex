use deadpool::managed::Timeouts;
// use deadpool::managed::{self,};
use deadpool_runtime::Runtime;
use diesel_async::pg::AsyncPgConnection;
use diesel_async::pooled_connection::{
    deadpool::{Object, Pool},
    AsyncDieselConnectionManager,
};

use std::sync::Arc;

pub struct AppState {
    pub db_pool: Arc<Pool<AsyncPgConnection>>,
}

impl AppState {
    pub fn new(db_url: &str) -> Self {
        Self {
            db_pool: Arc::new(build_connection(db_url)),
        }
    }
}

pub fn build_connection(db_url: &str) -> Pool<AsyncPgConnection> {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);

    Pool::builder(manager)
        .max_size(10)
        .runtime(Runtime::Tokio1)
        .timeouts(Timeouts::wait_millis(5000))
        .build()
        .expect("Could not build connection pool")
}

pub async fn get_connection_from_pool(
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

#[derive(Debug)]
pub enum AsyncPoolError {
    Timeout,
}
