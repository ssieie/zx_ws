use serde::{Deserialize};
use actix_web::web;

#[derive(Deserialize, Debug, Clone)]
pub struct Pager {
    pub page: i64,

    #[serde(rename = "pageSize")]
    pub page_size: i64,
}

impl From<web::Json<Pager>> for Pager {
    fn from(page: web::Json<Pager>) -> Self {
        Self {
            page: page.page,
            page_size: page.page_size,
        }
    }
}
