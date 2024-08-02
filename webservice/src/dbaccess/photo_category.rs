use crate::errors::MyError;
use sqlx::postgres::PgPool;
use crate::common::api_response::ApiResponse;
use sqlx::Error as SQLxError;
use crate::models::admin::{Category, CreateCategory, UpdateCategory};
use chrono::{Local};

pub async fn get_photo_category_db(
    pool: &PgPool,
    data: CreateCategory,
) -> Result<ApiResponse<Vec<Category>>, MyError> {
    let search_pattern = format!("%{}%", data.category_name);
    let rows = sqlx::query_as!(
        Category,
        r#"SELECT * FROM public.photo_category where category_name ILIKE $1 ORDER BY create_time DESC"#,
        search_pattern
    )
        .fetch_all(pool)
        .await?;
    Ok(ApiResponse::success(rows, "获取成功"))
}

pub async fn add_photo_category_db(
    pool: &PgPool,
    data: CreateCategory,
) -> Result<ApiResponse<Category>, MyError> {
    let rows = sqlx::query!(
        r#"SELECT category_name FROM public.photo_category"#)
        .fetch_all(pool)
        .await?;

    let category_names: Vec<String> = rows.into_iter().map(|name| name.category_name).collect();

    if category_names.contains(&data.category_name) {
        return Err(MyError::CustomError("分类已存在".into()));
    }


    let create_time = Local::now().naive_local();
    let row = sqlx::query_as!(
        Category,
        r#"INSERT INTO public.photo_category (category_name, create_time, update_time) VALUES ($1, $2, $3) RETURNING id, category_name, create_time, update_time"#,
        data.category_name,
        create_time,
        create_time)
        .fetch_one(pool)
        .await?;

    Ok(ApiResponse::success(row, "新增成功"))
}

pub async fn update_photo_category_db(
    pool: &PgPool,
    data: UpdateCategory,
) -> Result<ApiResponse<Category>, MyError> {
    let update_time = Local::now().naive_local();
    let row = sqlx::query_as!(
        Category,
        "UPDATE public.photo_category set category_name = $1, update_time = $2 where id = $3 RETURNING id, category_name, create_time, update_time",
        data.category_name,
        update_time,
        data.id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| match e {
            SQLxError::RowNotFound => MyError::CustomError("类别ID不存在".into()),
            _ => MyError::DBError("更新失败".into())
        })?;

    if let Some(category) = row {
        Ok(ApiResponse::success(category, "修改成功"))
    } else {
        Err(MyError::NotFound("类别ID不存在".into()))
    }
}

pub async fn del_photo_category_db(
    pool: &PgPool,
    id: i32,
) -> Result<ApiResponse<String>, MyError> {
    let photo_category_id_total = sqlx::query_scalar!(r#"SELECT COUNT(*) FROM public.photo where photo_category_id = $1"#, id)
        .fetch_one(pool).await?;

    if let Some(total) = photo_category_id_total {
        if total > 0 {
            return Ok(ApiResponse::custom(None, "删除失败, 此分类已有照片引用", 400));
        }
    }

    sqlx::query!(r#"DELETE FROM public.photo_category where id = $1"#, id)
        .execute(pool)
        .await
        .map_err(|e| match e {
            SQLxError::RowNotFound => MyError::CustomError("类别ID不存在".into()),
            _ => MyError::DBError("删除失败".into())
        })?;

    Ok(ApiResponse::success("".into(), "删除成功"))
}
