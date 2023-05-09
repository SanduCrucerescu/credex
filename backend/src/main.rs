use backend::{
    db::db::AppState,
    server::{self, handlers::clients::clt_controler::ClientControler},
};

use diesel::{self, associations::HasTable, prelude::*};
use dotenvy::dotenv;
use std::{env, sync::Arc};
use tower_http::{services::ServeDir, trace::TraceLayer};

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     dotenv().ok();
//     let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     let pool = get_pool(&db_url);
//     let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

//     HttpServer::new(move || {
//         let cors = Cors::permissive();
//         // .allowed_origin("http://localhost:3000")
//         // .allowed_methods(vec!["GET", "POST"])
//         // .allowed_headers(vec![
//         //     header::AUTHORIZATION,
//         //     header::ACCEPT,
//         //     header::CONTENT_TYPE,
//         // ])
//         // .supports_credentials();
//         App::new()
//             .app_data(Data::new(AppState {
//                 db: db_addr.clone(),
//             }))
//             .service(
//                 scope("/api")
//                     .service(get_clients)
//                     .service(get_client)
//                     .service(get_user_transactions)
//                     .service(post_login)
//                     .service(post_user_transactions),
//             )
//             .wrap(cors)
//             .wrap(Logger::default())
//     })
//     .bind(("127.0.0.1", 8000))?
//     .run()
//     .await
// }
use axum::{extract::State, http::header::CONTENT_TYPE, routing::get, Extension, Json, Router};
use std::net::SocketAddr;

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
        // .with_state(Arc::new(AppState::new(&db_url)))
        .layer(Extension::<Arc<AppState>>(Arc::new(AppState::new(&db_url))))
        .layer(TraceLayer::new_for_http())
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin(
                    "http://localhost:8000"
                        .parse::<axum::http::HeaderValue>()
                        .unwrap(),
                )
                .allow_headers([CONTENT_TYPE])
                .allow_methods([axum::http::Method::GET, axum::http::Method::POST]),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Server started!");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}
