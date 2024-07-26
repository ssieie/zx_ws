use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use std::io;
use sqlx::postgres::PgPoolOptions;

#[path = "../dbaccess/mod.rs"]
mod dbaccess;
#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../models/mod.rs"]
mod models;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;
#[path = "../errors.rs"]
mod errors;

use routers::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("未找到 DATABASE_URL");
    let pg_pool = PgPoolOptions::new().connect(&database_url).await
    .unwrap();

    let shared_data = web::Data::new(AppState {
        db: pg_pool,
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(web_routes)
            .configure(admin_routes)
    };
    println!("HttpServer Run In 0.0.0.0:9000");
    HttpServer::new(app).bind("0.0.0.0:9000")?.run().await
}
