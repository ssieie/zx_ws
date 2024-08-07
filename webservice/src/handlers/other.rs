use crate::errors::MyError;
use actix_web::{HttpResponse};
use awc::Client;
use std::time::Duration;

pub async fn get_60s_news_list() -> Result<HttpResponse, MyError> {
    let client = Client::default();

    let url = "https://api.03c3.cn/api/zb";

    match client.get(url).timeout(Duration::from_secs(60)).send().await {
        Ok(mut response) => {
            if response.status().is_success() {
                // 获取图片的字节流
                let image_bytes = response.body().await.map_err(|e| {
                    MyError::CustomError(format!("读取响应体失败: {:?}", e))
                })?;

                Ok(HttpResponse::Ok().content_type("image/jpeg").body(image_bytes))
            } else {
                return Err(MyError::CustomError("60s news获取失败".into()));
            }
        }
        Err(_e) => {
            return Err(MyError::CustomError("60s news获取失败".into()))
        }
    }
}
