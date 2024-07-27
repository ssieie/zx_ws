use actix_web::web;
use crate::errors::MyError;
use sqlx::postgres::PgPool;
use crate::models::login::{Login};
use uuid::Uuid;
use sqlx::Error as SQLxError;
use crate::common::api_response::ApiResponse;
use crate::state::AppState;

pub async fn valid_login_db(
    app_state: web::Data<AppState>,
    info: Login,
) -> Result<ApiResponse<String>, MyError> {
    let pool: &PgPool = &app_state.db;
    let user = sqlx::query!(r#"SELECT password FROM public.user where username = $1"#,info.username)
        .fetch_one(pool)
        .await
        .map_err(|e| match e {
            SQLxError::RowNotFound => MyError::CustomError(format!("用户名 '{}' 不存在", info.username)),
            _ => MyError::DBError("无法获取用户数据".into()),
        })?;


    if user.password == info.password {
        // 成功
        let token = Uuid::new_v4().to_string();

        let mut authorization = app_state.authorization.lock().unwrap();
        *authorization = token.clone();

        Ok(ApiResponse::success(token, "登录成功"))
    } else {
        Err(MyError::CustomError("密码错误".into()))
    }
}