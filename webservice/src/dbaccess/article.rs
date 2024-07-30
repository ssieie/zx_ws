use crate::errors::MyError;
use sqlx::postgres::PgPool;
use crate::common::api_response::ApiResponse;
use sqlx::Error as SQLxError;
use crate::models::admin::{Article, ArticleList, ArticleSimpleList, CreateArticle, HotCategory, UpdateArticle};
use chrono::{Local};

pub async fn get_article_list_db(
    pool: &PgPool,
    data: CreateArticle,
) -> Result<ApiResponse<Vec<ArticleList>>, MyError> {
    let search_pattern = if data.title.trim().is_empty() {
        "%".to_string()
    } else {
        format!("%{}%", data.title)
    };
    let search_pattern_desc = if data.describe.trim().is_empty() {
        "%".to_string()
    } else {
        format!("%{}%", data.describe)
    };

    let rows = sqlx::query_as!(
        ArticleList,
        r#"SELECT id,c_id,c_name,title,describe,heat,like_number,create_time,update_time
           FROM public.article
           where title ILIKE $1
           and describe ILIKE $2
           and (c_id = $3 OR $3 IS NULL)
           ORDER BY create_time DESC"#,
        search_pattern,
        search_pattern_desc,
        data.c_id
    )
        .fetch_all(pool)
        .await?;

    Ok(ApiResponse::success(rows, "获取成功"))
}

pub async fn get_article_db(
    pool: &PgPool,
    id: i32,
) -> Result<ApiResponse<Article>, MyError> {
    let row = sqlx::query_as!(
        Article,
        r#"SELECT * FROM public.article where id = $1"#,
        id
    )
        .fetch_one(pool)
        .await?;
    Ok(ApiResponse::success(row, "获取成功"))
}

pub async fn add_article_db(
    pool: &PgPool,
    data: CreateArticle,
) -> Result<ApiResponse<Article>, MyError> {
    let create_time = Local::now().naive_local();
    let row = sqlx::query_as!(
        Article,
        r#"INSERT INTO public.article (c_id, c_name, title, describe, text, create_time, update_time)
        VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id, c_id, c_name, title, describe, text, heat, like_number, create_time, update_time"#,
        data.c_id,
        data.c_name,
        data.title,
        data.describe,
        data.text,
        create_time,
        create_time)
        .fetch_one(pool)
        .await?;

    Ok(ApiResponse::success(row, "新增成功"))
}

pub async fn update_article_db(
    pool: &PgPool,
    data: UpdateArticle,
) -> Result<ApiResponse<Article>, MyError> {
    let update_time = Local::now().naive_local();
    let row = sqlx::query_as!(
        Article,
        r#"UPDATE public.article set c_id = $1, c_name = $2, title = $3, describe = $4, text = $5, update_time = $6
        where id = $7
        RETURNING id, c_id, c_name, title, describe, text, heat, like_number, create_time, update_time"#,
        data.c_id,
        data.c_name,
        data.title,
        data.describe,
        data.text,
        update_time,
        data.id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| match e {
            SQLxError::RowNotFound => MyError::CustomError("文章ID不存在".into()),
            _ => MyError::DBError("更新失败".into())
        })?;

    if let Some(category) = row {
        Ok(ApiResponse::success(category, "修改成功"))
    } else {
        Err(MyError::NotFound("文章ID不存在".into()))
    }
}

pub async fn del_article_db(
    pool: &PgPool,
    id: i32,
) -> Result<ApiResponse<String>, MyError> {
    sqlx::query!(r#"DELETE FROM public.article where id = $1"#, id)
        .execute(pool)
        .await
        .map_err(|e| match e {
            SQLxError::RowNotFound => MyError::CustomError("文章ID不存在".into()),
            _ => MyError::DBError("删除失败".into())
        })?;

    Ok(ApiResponse::success("".into(), "删除成功"))
}

pub async fn update_article_heat_db(
    pool: &PgPool,
    id: i32,
) -> Result<(), MyError> {
    sqlx::query!(r#"UPDATE public.article SET heat = heat + 1 where id = $1"#, id)
        .execute(pool)
        .await
        .map_err(|e| match e {
            SQLxError::RowNotFound => MyError::CustomError("文章ID不存在".into()),
            _ => MyError::DBError("删除失败".into())
        })?;

    Ok(())
}

//

pub async fn get_article_list_web_db(
    pool: &PgPool,
) -> Result<ApiResponse<Vec<ArticleList>>, MyError> {
    let rows = sqlx::query_as!(
        ArticleList,
        r#"SELECT id,c_id,c_name,title,describe,heat,like_number,create_time,update_time
           FROM public.article
           ORDER BY create_time DESC, update_time DESC
           LIMIT 20"#
    )
        .fetch_all(pool)
        .await?;

    Ok(ApiResponse::success(rows, "获取成功"))
}

pub async fn get_article_list_all_web_db(
    pool: &PgPool,
    cid: Option<i32>,
) -> Result<ApiResponse<Vec<ArticleList>>, MyError> {
    let rows = sqlx::query_as!(
        ArticleList,
        r#"SELECT id,c_id,c_name,title,describe,heat,like_number,create_time,update_time
           FROM public.article
           WHERE ($1::integer IS NULL OR c_id = $1)
           ORDER BY create_time DESC, update_time DESC"#,
        cid
    )
        .fetch_all(pool)
        .await?;

    Ok(ApiResponse::success(rows, "获取成功"))
}

pub async fn get_article_hot_list_web_db(
    pool: &PgPool,
) -> Result<ApiResponse<Vec<ArticleSimpleList>>, MyError> {
    let rows = sqlx::query_as!(
        ArticleSimpleList,
        r#"SELECT id,title,create_time,update_time
           FROM public.article
           ORDER BY heat DESC, like_number DESC
           LIMIT 10"#
    )
        .fetch_all(pool)
        .await?;

    Ok(ApiResponse::success(rows, "获取成功"))
}

pub async fn get_hot_category_list_db(
    pool: &PgPool,
) -> Result<ApiResponse<Vec<HotCategory>>, MyError> {
    let rows = sqlx::query_as!(
        HotCategory,
        r#"SELECT c_id, c_name, SUM(heat) AS total_heat
           FROM public.article
           GROUP BY c_id, c_name
           ORDER BY total_heat DESC
           LIMIT 10"#
    )
        .fetch_all(pool)
        .await?;

    Ok(ApiResponse::success(rows, "获取成功"))
}