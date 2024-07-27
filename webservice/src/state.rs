use sqlx::postgres::PgPool;
use std::sync::Mutex;

pub struct AppState {
    pub authorization: Mutex<String>,
    pub db: PgPool,
}
