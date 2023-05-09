use backend::db::{
    db::{build_connection, AppState},
    db_models::Client,
    schema::clients::dsl::*,
};

use diesel::{self, associations::HasTable, prelude::*};
// use actix::SyncArbiter;
// use actix_cors::Cors;
// use actix_web::{
//     middleware::Logger,
//     web::{scope, Data},
//     App, HttpServer,
// };
// use backend::{
//     db::db_utils::{get_pool, AppState, DbActor},
//     server::handlers::user_handler::{
//         get_client, get_clients, get_user_transactions, post_login, post_user_transactions,
//     },
// };
use dotenvy::dotenv;
use std::{env, sync::Arc};

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
use axum::{extract::State, http::header::CONTENT_TYPE, routing::get, Json, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let state = Arc::new(AppState {
        db_pool: Arc::new(build_connection(&db_url)),
    });

    let app = Router::new()
        .route("/", get(handler))
        .with_state(state)
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

async fn handler(State(state): State<Arc<AppState>>) {
    let db = state.as_ref().db_pool.clone();
    let conn = db.get().await;

    // let res = clients
    //     .filter(client_id.eq("C001"))
    //     .get_result::<Client>(&mut conn);

    // let db = state.as_ref().db_pool.clone();

    // let mut conn = db.get();
    // // .await
    // // .expect("Fetch data: Unable to establish connection.");

    // clients::table.count().get_results(&db);
}
