use backend::db::db::AppState;

use axum::{http::header::CONTENT_TYPE, Extension, Router};
use backend::server;
use dotenvy::dotenv;
use std::net::SocketAddr;
use std::{env, sync::Arc};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let app = Router::new()
        .nest(
            "/api",
            Router::new().nest(
                "/client",
                server::handlers::clients::clt_routes::clt_routes(),
            ),
        )
        .layer(Extension::<Arc<AppState>>(Arc::new(AppState::new(&db_url))))
        .layer(CorsLayer::permissive())
        // .layer(
        //     tower_http::cors::CorsLayer::new()
        //         .allow_origin(
        //             "http://127.0.0.1:8000"
        //                 .parse::<axum::http::HeaderValue>()
        //                 .unwrap(),
        //         )
        //         .allow_headers([CONTENT_TYPE])
        //         .allow_methods([axum::http::Method::GET, axum::http::Method::POST]),
        // )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Server started!");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}
