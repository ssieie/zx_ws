use serde::{Deserialize, Serialize};
use actix_web::web;
use chrono::NaiveDateTime;
use crate::models::common::Pager;

#[derive(Deserialize, Debug, Clone)]
pub struct RequestLogParams {
    pub uri: String,
    pub method: String,
    pub page: Pager,
}

impl From<web::Json<RequestLogParams>> for RequestLogParams {
    fn from(req: web::Json<RequestLogParams>) -> Self {
        Self {
            uri: req.uri.clone(),
            method: req.method.clone(),
            page: req.page.clone(),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct ResRequestLog {
    pub data: Vec<NewRequestLog>,
    pub total: i64,
}

impl ResRequestLog {
    pub fn new(data: Vec<NewRequestLog>, total: i64) -> Self {
        Self {
            data,
            total,
        }
    }
}

#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct NewRequestLog {
    pub uri: String,
    pub method: String,
    #[serde(rename = "ipAddress")]
    pub ip_address: Option<String>,
    #[serde(rename = "userAgent")]
    pub user_agent: Option<String>,
    pub time: Option<NaiveDateTime>,
}
