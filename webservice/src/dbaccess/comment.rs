use crate::errors::MyError;
use sqlx::postgres::PgPool;
use crate::common::api_response::ApiResponse;
use crate::models::comment::{CreateComment, Comment, CommentRes};
use chrono::{Local};
use crate::utils::tree::{Tree};
use std::env;

use std::error::Error;

use lettre::{Message, SmtpTransport, Transport};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;

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

    // 是否需要发送邮件提醒
    if let Some(parent_id) = data.p_id {
        match reply_email_notice(&pool, parent_id, &data.comment, &data.name).await {
            Ok(..) => {}
            Err(..) => {}
        };
    }

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

    return Ok(ApiResponse::success(CommentRes::new(tree, total_count.unwrap()), "获取成功"));
}

async fn reply_email_notice(pool: &PgPool, parent_id: i32, comment: &str, name: &str) -> Result<(), Box<dyn Error>> {
    let row = sqlx::query!(r#"SELECT call, email FROM public.comment where id = $1"#, parent_id)
        .fetch_one(pool)
        .await?;

    if row.call == "1" && !row.email.is_empty() {
        let email_addr = env::var("EMAIL")?;
        let password = env::var("EMAIL_PASSWORD")?;

        let email = Message::builder()
            .from(email_addr.parse()?)
            .to(row.email.parse()?)
            .subject("Zx_Blog 有人回复了你的评论")
            .header(ContentType::TEXT_PLAIN)
            .body(format!("{} 回复了你的评论: {}", name, comment))?;

        // 设置SMTP服务器的凭据
        let creds = Credentials::new(email_addr, password);

        let mailer = SmtpTransport::relay("smtp.qq.com")?
            .credentials(creds)
            .build();

        // 发送邮件
        match mailer.send(&email) {
            Ok(response) => {
                println!("Email sent successfully: {:?}", response);
            }
            Err(e) => {
                eprintln!("Could not send email: {:?}", e);
            }
        }
    }


    Ok(())
}