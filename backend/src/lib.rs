use surrealdb::{engine::remote::ws::Client, Surreal};

pub mod db;
pub mod server;
pub static DBS: Surreal<Client> = Surreal::init();
