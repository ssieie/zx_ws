use crate::errors::MyError;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use crate::dbaccess::category::*;
use crate::models::admin::{CreateCategory, UpdateCategory};

pub async fn get_category(
    app_state: web::Data<AppState>,
    data: web::Json<CreateCategory>,
) -> Result<HttpResponse, MyError> {
    get_category_db(&app_state.db, data.into()).await.map(|res| HttpResponse::Ok().json(res))
}

pub async fn add_category(
    app_state: web::Data<AppState>,
    data: web::Json<CreateCategory>,
) -> Result<HttpResponse, MyError> {
    add_category_db(&app_state.db, data.into()).await.map(|res| HttpResponse::Ok().json(res))
}

pub async fn update_category(
    app_state: web::Data<AppState>,
    data: web::Json<UpdateCategory>,
) -> Result<HttpResponse, MyError> {
    update_category_db(&app_state.db, data.into()).await.map(|res| HttpResponse::Ok().json(res))
}

pub async fn delete_category(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let id = params.into_inner();
    del_category_db(&app_state.db, id).await.map(|res| HttpResponse::Ok().json(res))
}
