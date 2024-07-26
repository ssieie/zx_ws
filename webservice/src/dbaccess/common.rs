use crate::errors::MyError;
use sqlx::postgres::PgPool;
use crate::models::login::{Login};
use uuid::Uuid;
use sqlx::Error as SQLxError;
use crate::common::api_response::ApiResponse;

pub async fn valid_login_db(
    pool: &PgPool,
    info: Login,
) -> Result<ApiResponse<String>, MyError> {
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

        let res = sqlx::query!(r#"UPDATE public.user SET token = $1 where username = $2"#,token,info.username)
            .execute(pool)
            .await
            .map_err(|e| match e {
                _ => MyError::DBError("数据库未知错误".into()),
            })?;

        if res.rows_affected() > 0 {
            Ok(ApiResponse::success(token, "登录成功"))
        } else {
            Err(MyError::NotFound("用户未找到或未更新任何行".into()))
        }
    } else {
        Err(MyError::CustomError("密码错误".into()))
    }
}