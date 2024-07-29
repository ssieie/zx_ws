use chrono::Local;
use sqlx::PgPool;
use crate::common::api_response::ApiResponse;
use crate::errors::MyError;
use sqlx::Error as SQLxError;
use crate::models::admin::{Introduce, CreateIntroduce};

pub async fn get_introduce_list_db(
    pool: &PgPool,
    data: CreateIntroduce,
) -> Result<ApiResponse<Vec<Introduce>>, MyError> {
    let search_pattern = format!("%{}%", data.text);
    let rows = sqlx::query_as!(
        Introduce,
        r#"SELECT * FROM public.introduce where text ILIKE $1 ORDER BY create_time DESC"#,
        search_pattern
    )
        .fetch_all(pool)
        .await?;
    Ok(ApiResponse::success(rows, "获取成功"))
}

pub async fn add_introduce_db(
    pool: &PgPool,
    data: CreateIntroduce,
) -> Result<ApiResponse<Introduce>, MyError> {
    let rows = sqlx::query!(
        r#"SELECT text FROM public.introduce"#)
        .fetch_all(pool)
        .await?;

    let texts: Vec<String> = rows.into_iter().map(|name| name.text).collect();

    if texts.contains(&data.text) {
        return Err(MyError::CustomError("记录重复".into()));
    }


    let create_time = Local::now().naive_local();
    let row = sqlx::query_as!(
        Introduce,
        r#"INSERT INTO public.introduce (text, create_time, update_time) VALUES ($1, $2, $3) RETURNING id, text, create_time, update_time"#,
        data.text,
        create_time,
        create_time)
        .fetch_one(pool)
        .await?;

    Ok(ApiResponse::success(row, "新增成功"))
}

pub async fn update_introduce_db(
    pool: &PgPool,
    data: Introduce,
) -> Result<ApiResponse<Introduce>, MyError> {
    let update_time = Local::now().naive_local();
    let row = sqlx::query_as!(
        Introduce,
        "UPDATE public.introduce set text = $1, update_time = $2 where id = $3 RETURNING id, text, create_time, update_time",
        data.text,
        update_time,
        data.id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| match e {
            SQLxError::RowNotFound => MyError::CustomError("ID不存在".into()),
            _ => MyError::DBError("更新失败".into())
        })?;

    if let Some(introduce) = row {
        Ok(ApiResponse::success(introduce, "修改成功"))
    } else {
        Err(MyError::NotFound("ID不存在".into()))
    }
}

pub async fn delete_introduce_db(
    pool: &PgPool,
    id: i32,
) -> Result<ApiResponse<String>, MyError> {
    let execute = sqlx::query!(r#"DELETE FROM public.introduce where id = $1"#, id)
        .execute(pool)
        .await
        .map_err(|e| match e {
            SQLxError::RowNotFound => MyError::CustomError("ID不存在".into()),
            _ => MyError::DBError("删除失败".into())
        })?;

    if execute.rows_affected() > 0 {
        Ok(ApiResponse::success("".into(), "删除成功"))
    } else {
        Err(MyError::NotFound("ID不存在".into()))
    }
}