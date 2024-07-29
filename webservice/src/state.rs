use sqlx::postgres::PgPool;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use chrono::{Utc, DateTime};

// 定义一个内存中的 IP 访问记录, 只用来实现文章热度更合理
type IpArticleAccessMap  = Arc<Mutex<HashMap<(String, i32), DateTime<Utc>>>>;

pub struct AppState {
    pub authorization: Mutex<String>, // 简单实现后台验证
    pub ip_article_access_map: IpArticleAccessMap ,
    pub db: PgPool,
    pub _cleanup_handle: tokio::task::JoinHandle<()>, // 定时清理任务
}
