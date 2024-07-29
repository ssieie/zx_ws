use std::future::{ready, Ready};

use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, web};
use chrono::Local;
use futures_util::future::LocalBoxFuture;
use crate::state::AppState;

pub struct RequestRecord;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for RequestRecord
where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequestRecordMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestRecordMiddleware { service }))
    }
}

pub struct RequestRecordMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestRecordMiddleware<S>
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
        let ip = req
            .peer_addr()
            .map(|addr| addr.ip().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        let user_agent = req
            .headers()
            .get("User-Agent")
            .and_then(|ua| ua.to_str().ok())
            .unwrap_or("Unknown").to_string();

        let timestamp = Local::now().naive_local();

        let method = req.method().to_string();

        let uri = req.uri().to_string();

        let app_data = req.app_data::<web::Data<AppState>>().cloned();

        let fut = self.service.call(req);

        Box::pin(async move {
            // 等待响应
            let res = fut.await?;

            if !uri.starts_with("/api/requestLog") {
                if let Some(state) = app_data {
                    sqlx::query!(r#"INSERT INTO public.request_log (ip_address, user_agent, time, method, uri)
                VALUES ($1, $2, $3, $4, $5)"#,
                ip,user_agent,timestamp,method,uri)
                        .execute(&state.db)
                        .await
                        .expect("访问日志插入错误");
                }
            }

            Ok(res)
        })
    }
}