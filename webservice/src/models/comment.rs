use serde::{Deserialize, Serialize};
use actix_web::web::Json;
use chrono::NaiveDateTime;
use crate::utils::tree::{Node, TreeNode};

#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Comment {
    pub id: i32,

    #[serde(rename = "pId")]
    pub p_id: Option<i32>,

    #[serde(rename = "articleId")]
    pub article_id: i32,

    pub name: String,

    pub comment: String,

    #[serde(rename = "createTime")]
    pub create_time: Option<NaiveDateTime>,
}

impl TreeNode for Comment {
    type Id = i32;
    fn key(&self) -> Self::Id {
        self.id
    }
    fn parent_key(&self) -> Option<Self::Id> {
        self.p_id
    }
}

#[derive(Serialize)]
pub struct CommentRes {
    pub data: Option<Vec<Node<Comment>>>,
    pub total: i64,
}

impl CommentRes {
    pub fn new(data: Option<Vec<Node<Comment>>>, total: i64) -> Self {
        Self {
            data,
            total,
        }
    }
}

impl From<Json<Comment>> for Comment {
    fn from(comment: Json<Comment>) -> Self {
        Self {
            id: comment.id,
            p_id: comment.p_id,
            article_id: comment.article_id,
            name: comment.name.clone(),
            comment: comment.comment.clone(),
            create_time: comment.create_time,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateComment {
    #[serde(rename = "pId")]
    pub p_id: Option<i32>,

    #[serde(rename = "articleId")]
    pub article_id: i32,

    pub call: String,

    pub name: String,

    pub comment: String,

    pub email: String,

    pub token: String,
}
impl From<Json<CreateComment>> for CreateComment {
    fn from(comment: Json<CreateComment>) -> Self {
        Self {
            p_id: comment.p_id,
            article_id: comment.article_id,
            call: comment.call.clone(),
            name: comment.name.clone(),
            comment: comment.comment.clone(),
            email: comment.email.clone(),
            token: comment.email.clone(),
        }
    }
}