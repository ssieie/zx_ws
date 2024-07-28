use crate::errors::MyError;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use crate::dbaccess::article::*;
use crate::models::admin::{CreateArticle, UpdateArticle};

pub async fn get_article_list(
    app_state: web::Data<AppState>,
    data: web::Json<CreateArticle>,
) -> Result<HttpResponse, MyError> {
    get_article_list_db(&app_state.db, data.into()).await.map(|res| HttpResponse::Ok().json(res))
}

pub async fn get_article(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let id = params.into_inner();
    get_article_db(&app_state.db, id).await.map(|res| HttpResponse::Ok().json(res))
}

pub async fn add_article(
    app_state: web::Data<AppState>,
    data: web::Json<CreateArticle>,
) -> Result<HttpResponse, MyError> {
    add_article_db(&app_state.db, data.into()).await.map(|res| HttpResponse::Ok().json(res))
}

pub async fn update_article(
    app_state: web::Data<AppState>,
    data: web::Json<UpdateArticle>,
) -> Result<HttpResponse, MyError> {
    update_article_db(&app_state.db, data.into()).await.map(|res| HttpResponse::Ok().json(res))
}

pub async fn delete_article(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let id = params.into_inner();
    del_article_db(&app_state.db, id).await.map(|res| HttpResponse::Ok().json(res))
}

//
pub async fn get_article_list_web(
    app_state: web::Data<AppState>
) -> Result<HttpResponse, MyError> {
    get_article_list_web_db(&app_state.db).await.map(|res| HttpResponse::Ok().json(res))
}