use actix::SyncArbiter;
use actix_web::{get, post, web::Data, App, HttpResponse, HttpServer, Responder};
use credex::{
    db::db_utils::{get_pool, AppState, DbActor},
    server::handlers::user_handler::get_users,
};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenvy::dotenv;
use std::env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                db: db_addr.clone(),
            }))
            .service(get_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
