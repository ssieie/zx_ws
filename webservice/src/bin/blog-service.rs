use actix_web::{web, App, HttpServer, http, middleware as actixMiddleware};
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
#[allow(unused)]
use crate::config::config::{DEVELOPMENT_BUCKET_URL, PRODUCTION_BUCKET_URL};

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
#[path = "../utils/mod.rs"]
mod utils;
#[path = "../config/mod.rs"]
mod config;

const HTTP_ADDR: &str = "0.0.0.0:9000";

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("创建文件上传目录");
    #[cfg(windows)]
    {
        std::fs::create_dir_all(DEVELOPMENT_BUCKET_URL)?;
    }
    #[cfg(not(windows))]
    {
        std::fs::create_dir_all(PRODUCTION_BUCKET_URL)?;
    }

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
        authorization: Mutex::new(String::from("5d51f2a8-953c-4188-9348-342393404a4c")),
        _cleanup_handle: cleanup_handle,
    });

    let app = move || {
        App::new()
            .wrap(actixMiddleware::Logger::default())
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

    info!("在以下位置启动HTTP服务器 {}", HTTP_ADDR);

    HttpServer::new(app)
        .keep_alive(http::KeepAlive::Timeout(Duration::from_secs_f32(90.0)))
        .bind(HTTP_ADDR)
        .unwrap_or_else(|_err| {
            error!("could not bind server to address {}", HTTP_ADDR);
            error!("error : {}", _err.to_string());
            exit(-1)
        })
        .run()
        .await
}
