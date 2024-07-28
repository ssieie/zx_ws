use crate::errors::MyError;
use sqlx::postgres::PgPool;
use crate::common::api_response::ApiResponse;
use sqlx::Error as SQLxError;
use crate::models::admin::About;

pub async fn get_about_db(
    pool: &PgPool
) -> Result<ApiResponse<String>, MyError> {
    let info = sqlx::query!(r#"SELECT about FROM public.info"#)
        .fetch_one(pool)
        .await
        .map_err(|e| match e {
            SQLxError::RowNotFound => MyError::CustomError("个人详情数据为空".into()),
            _ => MyError::DBError("无法获取个人详情数据".into()),
        })?;

    match info.about {
        Some(about) => Ok(ApiResponse::success(about, "获取成功")),
        _ => Err(MyError::CustomError("个人详情查询失败".into()))
    }
}

pub async fn update_about_db(
    pool: &PgPool,
    about: About,
) -> Result<ApiResponse<String>, MyError> {
    let row = sqlx::query!(r#"UPDATE public.info set about = $1"#,about.content)
        .execute(pool)
        .await
        .map_err(|e| match e {
            _ => MyError::DBError("无法获取个人详情数据".into()),
        })?;

    if row.rows_affected() == 1 {
        Ok(ApiResponse::success("".into(), "更新成功"))
    } else {
        Err(MyError::DBError("更新失败，受影响的行数不为 1".into()))
    }
}