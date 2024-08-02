use crate::errors::MyError;
use sqlx::postgres::PgPool;
use crate::common::api_response::ApiResponse;
use sqlx::Error as SQLxError;
use crate::models::photo::{TvSeries, CreatTvSeries, UpdateTvSeries};
use chrono::{Local};

pub async fn get_photo_tv_series_db(
    pool: &PgPool,
    data: CreatTvSeries,
) -> Result<ApiResponse<Vec<TvSeries>>, MyError> {
    let search_pattern_name = if data.name.trim().is_empty() {
        "%".to_string()
    } else {
        format!("%{}%", data.name)
    };
    let search_pattern_author: String = if let Some(author) = &data.author {
        if author.trim().is_empty() {
            "%".to_string()
        } else {
            format!("%{}%", author)
        }
    } else {
        "%".to_string()
    };

    let rows = sqlx::query_as!(
        TvSeries,
        r#"SELECT * FROM public.tv_series
        where name ILIKE $1
        and author ILIKE $2
        ORDER BY create_time DESC"#,
        search_pattern_name,
        search_pattern_author
    )
        .fetch_all(pool)
        .await?;

    Ok(ApiResponse::success(rows, "获取成功"))
}

pub async fn get_photo_tv_series_details_db(
    pool: &PgPool,
    id: i32,
) -> Result<ApiResponse<TvSeries>, MyError> {

    let row = sqlx::query_as!(
        TvSeries,
        r#"SELECT * FROM public.tv_series
        where id = $1"#,
        id,
    )
        .fetch_one(pool)
        .await?;

    Ok(ApiResponse::success(row, "获取成功"))
}

pub async fn add_photo_tv_series_db(
    pool: &PgPool,
    data: CreatTvSeries,
) -> Result<ApiResponse<TvSeries>, MyError> {
    let rows = sqlx::query!(
        r#"SELECT name FROM public.tv_series"#)
        .fetch_all(pool)
        .await?;

    let category_names: Vec<String> = rows.into_iter().map(|record| record.name).collect();

    if category_names.contains(&data.name) {
        return Err(MyError::CustomError("影集已存在".into()));
    }


    let create_time = Local::now().naive_local();
    let row = sqlx::query_as!(
        TvSeries,
        r#"INSERT INTO public.tv_series
        (name, cover_image, preview_image, author, describe, create_time, update_time)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, name, cover_image, preview_image, author, describe, create_time, update_time"#,
        data.name,
        data.cover_image,
        data.preview_image,
        data.author,
        data.describe,
        create_time,
        create_time)
        .fetch_one(pool)
        .await?;

    Ok(ApiResponse::success(row, "新增成功"))
}

pub async fn update_photo_tv_series_db(
    pool: &PgPool,
    data: UpdateTvSeries,
) -> Result<ApiResponse<TvSeries>, MyError> {
    let update_time = Local::now().naive_local();
    let row = sqlx::query_as!(
        TvSeries,
        r#"UPDATE public.tv_series
        set name = $1, cover_image = $2, preview_image = $3, author = $4, describe = $5, update_time = $6
        where id = $7
        RETURNING id, name, cover_image, preview_image, author, describe, create_time, update_time"#,
        data.name,
        data.cover_image,
        data.preview_image,
        data.author,
        data.describe,
        update_time,
        data.id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| match e {
            SQLxError::RowNotFound => MyError::CustomError("影集ID不存在".into()),
            _ => MyError::DBError("更新失败".into())
        })?;

    if let Some(category) = row {
        Ok(ApiResponse::success(category, "修改成功"))
    } else {
        Err(MyError::NotFound("影集ID不存在".into()))
    }
}

pub async fn del_photo_tv_series_db(
    pool: &PgPool,
    id: i32,
) -> Result<ApiResponse<String>, MyError> {
    let tv_series_id_total = sqlx::query_scalar!(r#"SELECT COUNT(*) FROM public.photo where tv_series_id = $1"#, id)
        .fetch_one(pool).await?;

    if let Some(total) = tv_series_id_total {
        if total > 0 {
            return Ok(ApiResponse::custom(None, "删除失败, 此影集已有照片引用", 400));
        }
    }

    sqlx::query!(r#"DELETE FROM public.tv_series where id = $1"#, id)
        .execute(pool)
        .await
        .map_err(|e| match e {
            SQLxError::RowNotFound => MyError::CustomError("影集ID不存在".into()),
            _ => MyError::DBError("删除失败".into())
        })?;

    Ok(ApiResponse::success("".into(), "删除成功"))
}
