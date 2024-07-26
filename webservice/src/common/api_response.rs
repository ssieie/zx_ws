use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub message: String,
    pub status: u16,
}

impl<T> ApiResponse<T> {
    // 构造成功响应
    pub fn success(data: T, message: &str) -> Self {
        ApiResponse {
            data: Some(data),
            message: message.to_string(),
            status: 200,
        }
    }

    // 构造错误响应
    pub fn error(message: &str, status: u16) -> Self {
        ApiResponse {
            data: None,
            message: message.to_string(),
            status,
        }
    }
}
