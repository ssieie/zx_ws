use std::future::{ready, Ready};

use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, http::header::HeaderValue, http::StatusCode, Error, error, HttpResponse, web};
use futures_util::future::LocalBoxFuture;
use sqlx::types::JsonValue::Null;
use crate::common::api_response::ApiResponse;
use crate::state::AppState;
use sqlx::postgres::PgPool;

pub struct Auth;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if has_permission(&req) {
            // 有权限，继续执行后续中间件
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            // 没有权限，立即返回响应
            Box::pin(async move {
                // 鉴权失败，返回未授权的响应，停止后续中间件的调用
                Err(error::ErrorUnauthorized("Unauthorized"))
            })
        }
    }
}

fn has_permission(req: &ServiceRequest) -> bool {
    let app_state = match req.app_data::<web::Data<AppState>>() {
        Some(data) => data,
        None => {
            println!("应用程序状态未初始化");
            return false; // 如果无法获取状态，拒绝访问
        }
    };
    let value = HeaderValue::from_str("").unwrap();
    let token = req.headers().get("Authorization").unwrap_or(&value);
    if let Ok(token_str) = token.to_str() {
        if token_str.len() > 0 {
            // 检查 token 是否有效
            let auth_token = app_state.authorization.lock().unwrap();
            return *auth_token == token_str
        }
    }

    req.path() == "/login" || req.path() == "/health" || req.path().starts_with("/web")
}