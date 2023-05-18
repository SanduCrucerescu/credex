use surrealdb::{engine::remote::ws::Client, Surreal};

pub mod error;
pub mod server;
pub static DBS: Surreal<Client> = Surreal::init();
