use actix_web::{web, App, HttpServer, http};
use actix_cors::Cors;
use dotenv::dotenv;
use std::env;
use std::io;
use std::process::exit;
use std::time::Duration;
use sqlx::postgres::PgPoolOptions;
use log::{error, info};

#[path = "../dbaccess/mod.rs"]
mod dbaccess;
#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../models/mod.rs"]
mod models;
#[path = "../common/mod.rs"]
mod common;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;
#[path = "../errors.rs"]
mod errors;

use routers::*;
use state::AppState;

const HTTP_ADDR: &str = "0.0.0.0:9000";

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
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:8888")
                    .allowed_origin("http://localhost:9999")
                    .allowed_origin("https://www.zxandhy.top")
                    .allowed_origin("https://admin.zxandhy.top")
                    .allowed_methods(vec!["GET", "POST", "DELETE", "OPTIONS"])
                    .allowed_headers(vec!["Content-Type", "Authorization"])
                    .allowed_header(http::header::ACCEPT)
                    .supports_credentials()
            )
            .app_data(shared_data.clone())
            .configure(web_routes)
            .configure(admin_routes)
    };

    HttpServer::new(app)
        .keep_alive(http::KeepAlive::Timeout(Duration::from_secs_f32(90.0)))
        .bind(HTTP_ADDR)
        .and_then(|server| {
            info!("bind server to address {}", HTTP_ADDR);
            Ok(server)
        })
        .unwrap_or_else(|_err| {
            error!("could not bind server to address {}", HTTP_ADDR);
            error!("error : {}", _err.to_string());
            exit(-1)
        })
        .run()
        .await
}
