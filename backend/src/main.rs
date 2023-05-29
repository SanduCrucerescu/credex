use axum::Router;
use backend::server::handlers::clients::clt_routes::clt_routes;
use backend::server::handlers::trx::trx_routes::trx_routes;
use backend::{server, DBS};
use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let username = env::var("SurrealDB_USERNAME").expect("Username must be set");
    let pass = env::var("SurrealDB_PASSWORD").expect("Password must be set");
    let ns = env::var("SurrealDB_NS").expect("NS must be set");
    let db = env::var("SurrealDB_DB").expect("DB must be set");

    DBS.connect::<Ws>(db_url).await?;
    DBS.signin(Root {
        username: &username,
        password: &pass,
    })
    .await?;
    DBS.use_ns(ns).use_db(db).await?;

    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                .nest("/client", clt_routes())
                .nest("/trx", trx_routes()),
        )
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server started!");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
    Ok(())
}
