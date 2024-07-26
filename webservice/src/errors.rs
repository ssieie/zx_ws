use actix_web::{
    error,
    http::{self, StatusCode},
    HttpResponse,
};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;
use crate::common::api_response::ApiResponse;

#[derive(Debug, Serialize)]
pub enum MyError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    CustomError(String)
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl MyError {
    fn error_response(&self) -> String {
        match self {
            MyError::DBError(msg) => {
                println!("数据库错误:{:?}", msg);
                "数据库错误".into()
            }
            MyError::ActixError(msg) => {
                println!("服务器错误:{:?}", msg);
                "内部服务器错误".into()
            }
            MyError::NotFound(msg) => {
                println!("Not found error occurred:{:?}", msg);
                msg.into()
            }
            MyError::CustomError(msg) => {
                println!("Custom error:{:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for MyError {
    fn status_code(&self) -> http::StatusCode {
        match self {
            MyError::DBError(_msg) | MyError::ActixError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFound(_msg) => StatusCode::NOT_FOUND,
            MyError::CustomError(_msg) => StatusCode::BAD_REQUEST,
        }
    }
    fn error_response(&self) -> HttpResponse {
        let api_response = ApiResponse::<()>::error(&self.error_response(), self.status_code().as_u16());
        HttpResponse::build(self.status_code()).json(api_response)
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for MyError {
    fn from(value: actix_web::error::Error) -> Self {
        MyError::ActixError(value.to_string())
    }
}

impl From<SQLxError> for MyError {
    fn from(value: SQLxError) -> Self {
        MyError::DBError(value.to_string())
    }
}
