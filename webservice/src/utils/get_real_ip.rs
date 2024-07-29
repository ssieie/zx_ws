use actix_web::HttpRequest;
use actix_web::{dev::{ServiceRequest}};
use actix_web::http::header::HeaderMap;

pub trait ZxRequestHeaders {
    fn headers(&self) -> &HeaderMap;
    fn peer_addr(&self) -> Option<std::net::SocketAddr>;
}

impl ZxRequestHeaders for HttpRequest {
    fn headers(&self) -> &HeaderMap {
        self.headers()
    }

    fn peer_addr(&self) -> Option<std::net::SocketAddr> {
        self.peer_addr()
    }
}

impl ZxRequestHeaders for ServiceRequest {
    fn headers(&self) -> &HeaderMap {
        self.headers()
    }

    fn peer_addr(&self) -> Option<std::net::SocketAddr> {
        self.connection_info().realip_remote_addr()
            .and_then(|ip_str| ip_str.parse().ok())
            .map(|ip| std::net::SocketAddr::new(ip, 0))
    }
}

// 获取真实 IP 地址的函数
pub fn get_real_ip<T: ZxRequestHeaders>(req: &T) -> String {
    if let Some(forwarded_for) = req.headers().get("X-Forwarded-For") {
        if let Ok(forwarded_for_str) = forwarded_for.to_str() {
            if let Some(real_ip) = forwarded_for_str.split(',').next() {
                return real_ip.trim().to_string();
            }
        }
    }

    // 如果没有 X-Forwarded-For，尝试使用 X-Real-IP
    if let Some(real_ip) = req.headers().get("X-Real-IP") {
        if let Ok(real_ip_str) = real_ip.to_str() {
            return real_ip_str.trim().to_string();
        }
    }

    // 如果以上方法都失败，使用 peer_addr
    req.peer_addr()
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|| "Unknown".to_string())
}