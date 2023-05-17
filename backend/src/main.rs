use backend::db::db::AppState;

use axum::{http::header::CONTENT_TYPE, Extension, Router};
use backend::{server, DBS};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::{env, sync::Arc};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    //dotenv().ok();
    //let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    DBS.connect::<Ws>("127.0.0.1:8000").await?;
    DBS.signin(Root {
        username: "root",
        password: "123",
    })
    .await?;
    DBS.use_ns("credex").use_db("credex").await?;

    let app = Router::new()
        .nest(
            "/api",
            Router::new().nest(
                "/client",
                server::handlers::clients::clt_routes::clt_routes(),
            ),
        )
        //.layer(Extension::<Arc<AppState>>(Arc::new(AppState::new(&db_url))))
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
