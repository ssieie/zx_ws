use actix_web::{web, App, HttpServer, http};
use actix_cors::Cors;
use dotenv::dotenv;
use std::env;
use std::io;
use std::process::exit;
use std::time::Duration;
use sqlx::postgres::PgPoolOptions;
use log::{error, info};
use routers::*;
use state::AppState;
use crate::middleware::auth::{Auth};
use crate::middleware::request_record::RequestRecord;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::{interval_at, Duration as TokioDuration, Instant};

#[path = "../dbaccess/mod.rs"]
mod dbaccess;
#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../models/mod.rs"]
mod models;
#[path = "../middleware/mod.rs"]
mod middleware;
#[path = "../common/mod.rs"]
mod common;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;
#[path = "../errors.rs"]
mod errors;

const HTTP_ADDR: &str = "0.0.0.0:9000";

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("未找到 DATABASE_URL");
    let pg_pool = PgPoolOptions::new().connect(&database_url).await
        .unwrap();

    let ip_article_access_map = Arc::new(Mutex::new(HashMap::new()));
    let ip_article_access_map_clone = ip_article_access_map.clone();

    let cleanup_handle = tokio::spawn(async move {
        let midnight = Instant::now() + TokioDuration::from_secs(60 * 60 * 24);
        let mut interval = interval_at(midnight, TokioDuration::from_secs(60 * 60 * 24));

        loop {
            interval.tick().await;
            let mut map = ip_article_access_map_clone.lock().unwrap();
            map.clear();
        }
    });

    let shared_data = web::Data::new(AppState {
        db: pg_pool,
        ip_article_access_map,
        authorization: Mutex::new(String::from("93c522ac-9e80-4f7c-a3c5-4571662bca91")),
        _cleanup_handle: cleanup_handle,
    });

    let app = move || {
        App::new()
            .wrap(RequestRecord)
            .wrap(Auth)
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:8888")
                    .allowed_origin("http://localhost:9999")
                    .allowed_origin("https://www.zxandhy.top")
                    .allowed_origin("https://admin.zxandhy.top")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                    .allowed_headers(vec!["Content-Type", "Authorization"])
                    .allowed_header(http::header::ACCEPT)
                    .supports_credentials()
            )
            .app_data(shared_data.clone())
            .configure(health_routes)
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
