use serde::{Deserialize, Serialize};
use actix_web::web;
use crate::errors::MyError;
use chrono::NaiveDateTime;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct About {
    pub content: String,
}
impl TryFrom<web::Json<About>> for About {
    type Error = MyError;
    fn try_from(about: web::Json<About>) -> Result<Self, Self::Error> {
        Ok(About {
            content: about.content.clone(),
        })
    }
}

/*************Category*************/

#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Category {
    pub id: i32,

    #[serde(rename = "categoryName")]
    pub category_name: String,

    #[serde(rename = "createTime")]
    pub create_time: Option<NaiveDateTime>,

    #[serde(rename = "updateTime")]
    pub update_time: Option<NaiveDateTime>,
}
impl From<web::Json<Category>> for Category {
    fn from(category: web::Json<Category>) -> Self {
        Self {
            id: category.id,
            category_name: category.category_name.clone(),
            create_time: category.create_time,
            update_time: category.update_time,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateCategory {
    #[serde(rename = "categoryName")]
    pub category_name: String,
}
impl From<web::Json<CreateCategory>> for CreateCategory {
    fn from(category: web::Json<CreateCategory>) -> Self {
        Self {
            category_name: category.category_name.clone(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateCategory {
    pub id: i32,
    #[serde(rename = "categoryName")]
    pub category_name: String,
}
impl From<web::Json<UpdateCategory>> for UpdateCategory {
    fn from(category: web::Json<UpdateCategory>) -> Self {
        Self {
            id: category.id,
            category_name: category.category_name.clone(),
        }
    }
}

#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct HotCategory {
    #[serde(rename = "cId")]
    pub c_id: i32,

    #[serde(rename = "cName")]
    pub c_name: String,

    #[serde(rename = "totalHeat")]
    pub total_heat: Option<i64>,
}

/*************Article*************/

#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Article {
    pub id: i32,

    #[serde(rename = "cId")]
    pub c_id: i32,

    #[serde(rename = "cName")]
    pub c_name: String,

    pub title: String,

    pub describe: String,

    pub text: Option<String>,

    pub heat: Option<i32>,

    #[serde(rename = "likeNumber")]
    pub like_number: Option<i32>,

    #[serde(rename = "createTime")]
    pub create_time: Option<NaiveDateTime>,

    #[serde(rename = "updateTime")]
    pub update_time: Option<NaiveDateTime>,
}
impl From<web::Json<Article>> for Article {
    fn from(article: web::Json<Article>) -> Self {
        Self {
            id: article.id,
            c_id: article.c_id,
            c_name: article.c_name.clone(),
            title: article.title.clone(),
            describe: article.describe.clone(),
            text: article.text.clone(),
            heat: article.heat,
            like_number: article.like_number,
            create_time: article.create_time,
            update_time: article.update_time,
        }
    }
}

#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct ArticleList {
    pub id: i32,

    #[serde(rename = "cId")]
    pub c_id: i32,

    #[serde(rename = "cName")]
    pub c_name: String,

    pub title: String,

    pub describe: String,

    pub heat: i32,

    #[serde(rename = "likeNumber")]
    pub like_number: i32,

    #[serde(rename = "createTime")]
    pub create_time: Option<NaiveDateTime>,

    #[serde(rename = "updateTime")]
    pub update_time: Option<NaiveDateTime>,
}
impl From<web::Json<ArticleList>> for ArticleList {
    fn from(article: web::Json<ArticleList>) -> Self {
        Self {
            id: article.id,
            c_id: article.c_id,
            c_name: article.c_name.clone(),
            title: article.title.clone(),
            describe: article.describe.clone(),
            heat: article.heat,
            like_number: article.like_number,
            create_time: article.create_time,
            update_time: article.update_time,
        }
    }
}

#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct ArticleSimpleList {
    pub id: i32,

    pub title: String,

    #[serde(rename = "createTime")]
    pub create_time: Option<NaiveDateTime>,

    #[serde(rename = "updateTime")]
    pub update_time: Option<NaiveDateTime>,
}
impl From<web::Json<ArticleSimpleList>> for ArticleSimpleList {
    fn from(article: web::Json<ArticleSimpleList>) -> Self {
        Self {
            id: article.id,
            title: article.title.clone(),
            create_time: article.create_time,
            update_time: article.update_time,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateArticle {
    #[serde(rename = "cId")]
    pub c_id: Option<i32>,

    #[serde(rename = "cName")]
    pub c_name: Option<String>,

    pub title: String,

    pub describe: String,

    pub text: Option<String>,
}
impl From<web::Json<CreateArticle>> for CreateArticle {
    fn from(article: web::Json<CreateArticle>) -> Self {
        Self {
            c_id: article.c_id,
            c_name: article.c_name.clone(),
            title: article.title.clone(),
            describe: article.describe.clone(),
            text: article.text.clone(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateArticle {
    pub id: i32,

    #[serde(rename = "cId")]
    pub c_id: i32,

    #[serde(rename = "cName")]
    pub c_name: String,

    pub title: String,

    pub describe: String,

    pub text: Option<String>,
}
impl From<web::Json<UpdateArticle>> for UpdateArticle {
    fn from(article: web::Json<UpdateArticle>) -> Self {
        Self {
            id: article.id,
            c_id: article.c_id,
            c_name: article.c_name.clone(),
            title: article.title.clone(),
            describe: article.describe.clone(),
            text: article.text.clone(),
        }
    }
}

/*************introduceManage*************/
#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Introduce {
    pub id: i32,

    pub text: String,

    #[serde(rename = "createTime")]
    pub create_time: Option<NaiveDateTime>,

    #[serde(rename = "updateTime")]
    pub update_time: Option<NaiveDateTime>,
}
impl From<web::Json<Introduce>> for Introduce {
    fn from(introduce: web::Json<Introduce>) -> Self {
        Self {
            id: introduce.id,
            text: introduce.text.clone(),
            create_time: introduce.create_time,
            update_time: introduce.update_time,
        }
    }
}

#[derive(Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct CreateIntroduce {
    pub text: String,
}
impl From<web::Json<CreateIntroduce>> for CreateIntroduce {
    fn from(introduce: web::Json<CreateIntroduce>) -> Self {
        Self {
            text: introduce.text.clone(),
        }
    }
}
