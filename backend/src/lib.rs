use surrealdb::{engine::remote::ws::Client, Surreal};

pub mod db;
pub mod error;
pub mod prelude;
pub mod server;
pub mod utils;
pub static DBS: Surreal<Client> = Surreal::init();
