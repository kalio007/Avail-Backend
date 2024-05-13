mod database;
mod lib;
mod routes;

use crate::lib::db_utils::{get_pool, AppState, DbActor};
use crate::routes::services::{create_user, create_user_article, get_user_articles, get_users, update_user};
use actix::SyncArbiter;
use actix_web::{web::Data, App, HttpServer};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenv::dotenv;
use routes::health_route::health_checker_handler;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&db_url);
    println!("✅ Connection to the db is successful!");
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    // starting the web server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                db: db_addr.clone(),
            }))
            .service(health_checker_handler)
            .service(get_users)
            .service(create_user)
            .service(update_user)
            .service(get_user_articles)
            .service(create_user_article)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
