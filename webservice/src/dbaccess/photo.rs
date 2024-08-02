use crate::errors::MyError;
use sqlx::postgres::PgPool;
use crate::common::api_response::ApiResponse;
use sqlx::Error as SQLxError;
use crate::models::photo::{Photo, CreatPhoto, UpdatePhoto};
use chrono::{Local};
use crate::models::common::Pager;

pub async fn get_photo_list_db(
    pool: &PgPool,
    data: CreatPhoto,
) -> Result<ApiResponse<Vec<Photo>>, MyError> {
    let author: String = if let Some(author) = &data.author {
        if author.trim().is_empty() {
            "%".to_string()
        } else {
            format!("%{}%", author)
        }
    } else {
        "%".to_string()
    };

    let rows = sqlx::query_as!(
        Photo,
        r#"SELECT
            p.id,
            p.photo_category_id,
            pc.category_name as photo_category_name,
            p.tv_series_id,
            ts.name as tv_series_name,
            p.photo_url,
            p.preview_url,
            p.author,
            p.like,
            p.width,
            p.height,
            p.create_time,
            p.update_time
        FROM public.photo p
        LEFT JOIN public.photo_category pc ON p.photo_category_id = pc.id
        LEFT JOIN public.tv_series ts ON p.tv_series_id = ts.id
        where p.author ILIKE $1
        and (p.photo_category_id = $2 OR $2 IS NULL)
        and (p.tv_series_id = $3 OR $3 IS NULL)
        ORDER BY create_time DESC"#,
        author,
        data.photo_category_id,
        data.tv_series_id,
    )
        .fetch_all(pool)
        .await?;

    Ok(ApiResponse::success(rows, "获取成功"))
}

pub async fn get_photo_details_db(
    pool: &PgPool,
    id: i32,
) -> Result<ApiResponse<Photo>, MyError> {
    let row = sqlx::query_as!(
        Photo,
        r#"SELECT
            p.id,
            p.photo_category_id,
            pc.category_name as photo_category_name,
            p.tv_series_id,
            ts.name as tv_series_name,
            p.photo_url,
            p.preview_url,
            p.author,
            p.like,
            p.width,
            p.height,
            p.create_time,
            p.update_time
        FROM public.photo p
        LEFT JOIN public.photo_category pc ON p.photo_category_id = pc.id
        LEFT JOIN public.tv_series ts ON p.tv_series_id = ts.id
        where p.id = $1"#,
        id,
    )
        .fetch_one(pool)
        .await?;

    Ok(ApiResponse::success(row, "获取成功"))
}

pub async fn add_photo_db(
    pool: &PgPool,
    data: CreatPhoto,
) -> Result<ApiResponse<&str>, MyError> {
    let create_time = Local::now().naive_local();
    let execute = sqlx::query!(
        r#"INSERT INTO public.photo
        (photo_category_id, tv_series_id, photo_url, preview_url, author, width, height, create_time, update_time)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
        data.photo_category_id,
        data.tv_series_id,
        data.photo_url,
        data.preview_url,
        data.author,
        data.width,
        data.height,
        create_time,
        create_time
        )
        .execute(pool)
        .await?;

    if execute.rows_affected() == 1 {
        Ok(ApiResponse::success("", "新增成功"))
    } else {
        Err(MyError::DBError("新增失败".into()))
    }
}

pub async fn update_photo_db(
    pool: &PgPool,
    data: UpdatePhoto,
) -> Result<ApiResponse<&str>, MyError> {
    let update_time = Local::now().naive_local();
    let execute = sqlx::query!(
        r#"UPDATE public.photo
        set photo_category_id = $1, tv_series_id = $2, photo_url = $3, preview_url = $4, author = $5, width = $6, height = $7, update_time = $8
        where id = $9"#,
        data.photo_category_id,
        data.tv_series_id,
        data.photo_url,
        data.preview_url,
        data.author,
        data.width,
        data.height,
        update_time,
        data.id
        )
        .execute(pool)
        .await
        .map_err(|e| match e {
            SQLxError::RowNotFound => MyError::CustomError("照片ID不存在".into()),
            _ => MyError::DBError("更新失败".into())
        })?;

    if execute.rows_affected() > 0 {
        Ok(ApiResponse::success("", "修改成功"))
    } else {
        Err(MyError::DBError("修改失败".into()))
    }
}

pub async fn del_photo_db(
    pool: &PgPool,
    id: i32,
) -> Result<ApiResponse<String>, MyError> {
    sqlx::query!(r#"DELETE FROM public.photo where id = $1"#, id)
        .execute(pool)
        .await
        .map_err(|e| match e {
            SQLxError::RowNotFound => MyError::CustomError("照片ID不存在".into()),
            _ => MyError::DBError("删除失败".into())
        })?;

    Ok(ApiResponse::success("".into(), "删除成功"))
}

////
pub async fn get_photo_list_hot_db(
    pool: &PgPool,
    page: Pager,
) -> Result<ApiResponse<Vec<Photo>>, MyError> {
    let rows = sqlx::query_as!(
        Photo,
        r#"SELECT
            p.id,
            p.photo_category_id,
            pc.category_name as photo_category_name,
            p.tv_series_id,
            ts.name as tv_series_name,
            p.photo_url,
            p.preview_url,
            p.author,
            p.like,
            p.width,
            p.height,
            p.create_time,
            p.update_time
        FROM public.photo p
        LEFT JOIN public.photo_category pc ON p.photo_category_id = pc.id
        LEFT JOIN public.tv_series ts ON p.tv_series_id = ts.id
        ORDER BY "like" DESC
        LIMIT $1 OFFSET $2"#,
        page.page_size,
        (page.page - 1) * page.page_size
    )
        .fetch_all(pool)
        .await?;

    Ok(ApiResponse::success(rows, "获取成功"))
}

pub async fn get_photo_list_new_db(
    pool: &PgPool,
    page: Pager,
) -> Result<ApiResponse<Vec<Photo>>, MyError> {
    let rows = sqlx::query_as!(
        Photo,
        r#"SELECT
            p.id,
            p.photo_category_id,
            pc.category_name as photo_category_name,
            p.tv_series_id,
            ts.name as tv_series_name,
            p.photo_url,
            p.preview_url,
            p.author,
            p.like,
            p.width,
            p.height,
            p.create_time,
            p.update_time
        FROM public.photo p
        LEFT JOIN public.photo_category pc ON p.photo_category_id = pc.id
        LEFT JOIN public.tv_series ts ON p.tv_series_id = ts.id
        ORDER BY create_time DESC
        LIMIT $1 OFFSET $2"#,
        page.page_size,
        (page.page - 1) * page.page_size
    )
        .fetch_all(pool)
        .await?;

    Ok(ApiResponse::success(rows, "获取成功"))
}