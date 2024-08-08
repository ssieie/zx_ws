use crate::errors::MyError;
use sqlx::postgres::PgPool;
use crate::common::api_response::ApiResponse;
use crate::models::comment::{CreateComment, Comment, CommentRes};
use chrono::{Local};
use crate::utils::tree::{Tree};

pub async fn add_article_comment_db(
    pool: &PgPool,
    data: CreateComment,
) -> Result<ApiResponse<Comment>, MyError> {
    let create_time = Local::now().naive_local();
    let row = sqlx::query_as!(
        Comment,
        r#"INSERT INTO public.comment
        (p_id, article_id, name, email, comment, call, create_time)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, p_id, article_id, name, comment, create_time"#,
        data.p_id,
        data.article_id,
        data.name,
        data.email,
        data.comment,
        data.call,
        create_time)
        .fetch_one(pool)
        .await?;

    Ok(ApiResponse::success(row, "新增成功"))
}

pub async fn get_article_comment_list_db(
    pool: &PgPool,
    id: i32,
) -> Result<ApiResponse<CommentRes>, MyError> {

    let total_count = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) FROM public.comment
        WHERE article_id = $1
        "#,
        id
    )
        .fetch_one(pool)
        .await?;

    let rows = sqlx::query_as!(
        Comment,
        r#"SELECT id, p_id, article_id, name, comment, create_time
        FROM public.comment where article_id = $1
        ORDER BY create_time DESC"#, id)
        .fetch_all(pool)
        .await?;

    let tree = Tree::from(rows).nodes;

    return Ok(ApiResponse::success(CommentRes::new(tree,total_count.unwrap()), "获取成功"));
}