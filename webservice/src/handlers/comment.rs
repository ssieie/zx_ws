use crate::errors::MyError;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use awc::Client;
use crate::dbaccess::comment::*;
use crate::models::comment::{CreateComment};
use std::collections::HashMap;
use std::env;
use std::time::Duration;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct VerificationResponse {
    success: bool,
    challenge_ts: Option<String>,
    hostname: Option<String>,
    #[serde(rename = "error-codes")]
    error_codes: Vec<String>,
    action: Option<String>,
    cdata: Option<String>,
}

pub async fn add_article_comment(
    app_state: web::Data<AppState>,
    data: web::Json<CreateComment>,
) -> Result<HttpResponse, MyError> {

    let client = Client::default();

    let url = "https://challenges.cloudflare.com/turnstile/v0/siteverify";

    if let Ok(secret) = env::var("TURNSTILE_SECRET") {
        let mut info = HashMap::new();
        info.insert("secret", &secret);
        info.insert("response", &data.token);
        
        match client.post(url).timeout(Duration::from_secs(60)).send_json(&info).await {
            Ok(mut response)=>{
                if response.status().is_success() {

                    let body = response.body().await.map_err(|e| {
                        MyError::CustomError(format!("验证读取响应体失败: {:?}", e))
                    })?;

                    let box_text = String::from_utf8(body.to_vec()).map_err(|e|{
                        MyError::CustomError(format!("响应体转String失败: {:?}", e))
                    })?;

                    let verification_response:VerificationResponse = serde_json::from_str(&box_text).map_err(|e|{
                        MyError::CustomError(format!("解析 JSON 失败: {:?}", e))
                    })?;

                    if verification_response.success {
                        add_article_comment_db(&app_state.db, data.into()).await.map(|res| HttpResponse::Ok().json(res))
                    }else {
                        return Err(MyError::CustomError("验证失败".into()));
                    }

                }else {
                    return Err(MyError::CustomError("验证失败".into()))
                }
            }
            Err(_e) => {
                return Err(MyError::CustomError("验证失败".into()))
            }
        }
    }else {
        return Err(MyError::CustomError("secret 获取失败".into()));
    }
}

pub async fn get_article_comment_list(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let id = params.into_inner();
    get_article_comment_list_db(&app_state.db, id).await.map(|res| HttpResponse::Ok().json(res))
}
