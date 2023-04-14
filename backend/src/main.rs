use actix::SyncArbiter;
use actix_cors::Cors;
use actix_web::{
    middleware::Logger,
    web::{scope, Data},
    App, HttpServer,
};
use backend::{
    db::db_utils::{get_pool, AppState, DbActor},
    server::handlers::user_handler::{
        get_client, get_clients, get_user_transactions, post_login, post_user_transactions,
    },
};
use dotenvy::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        let cors = Cors::permissive();
        // .allowed_origin("http://localhost:3000")
        // .allowed_methods(vec!["GET", "POST"])
        // .allowed_headers(vec![
        //     header::AUTHORIZATION,
        //     header::ACCEPT,
        //     header::CONTENT_TYPE,
        // ])
        // .supports_credentials();
        App::new()
            .app_data(Data::new(AppState {
                db: db_addr.clone(),
            }))
            .service(
                scope("/api")
                    .service(get_clients)
                    .service(get_client)
                    .service(get_user_transactions)
                    .service(post_login)
                    .service(post_user_transactions),
            )
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
