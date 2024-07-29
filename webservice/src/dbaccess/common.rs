use actix_web::web;
use crate::errors::MyError;
use sqlx::postgres::PgPool;
use crate::models::login::{Login};
use uuid::Uuid;
use sqlx::Error as SQLxError;
use crate::common::api_response::ApiResponse;
use crate::models::request_log::{NewRequestLog, RequestLogParams, ResRequestLog};
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

pub async fn request_log_db(
    pool: &PgPool,
    data: web::Json<RequestLogParams>,
) -> Result<ApiResponse<ResRequestLog>, MyError> {
    let search_path_pattern = if data.uri.trim().is_empty() {
        "%".to_string()
    } else {
        format!("%{}%", data.uri)
    };
    let search_method_pattern = if data.method.trim().is_empty() {
        "%".to_string()
    } else {
        format!("%{}%", data.method)
    };

    // 获取总行数
    let total_count = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) FROM public.request_log
        WHERE uri ILIKE $1 AND method ILIKE $2
        "#,
        search_path_pattern,
        search_method_pattern
    )
        .fetch_one(pool)
        .await?;

    let rows = sqlx::query_as!(
        NewRequestLog,
        r#"SELECT * FROM public.request_log
        where uri ILIKE $1 AND method ILIKE $2
        ORDER BY time DESC
        LIMIT $3 OFFSET $4"#,
        search_path_pattern, search_method_pattern, data.page.page_size, (data.page.page - 1) * data.page.page_size)
        .fetch_all(pool)
        .await?;
    Ok(ApiResponse::success(ResRequestLog::new(rows, total_count.unwrap()), "获取成功"))
}