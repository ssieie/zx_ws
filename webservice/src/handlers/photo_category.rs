use crate::errors::MyError;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use crate::dbaccess::photo_category::*;
use crate::models::admin::{CreateCategory, UpdateCategory};

pub async fn get_photo_category(
    app_state: web::Data<AppState>,
    data: web::Json<CreateCategory>,
) -> Result<HttpResponse, MyError> {
    get_photo_category_db(&app_state.db, data.into()).await.map(|res| HttpResponse::Ok().json(res))
}

pub async fn add_photo_category(
    app_state: web::Data<AppState>,
    data: web::Json<CreateCategory>,
) -> Result<HttpResponse, MyError> {
    add_photo_category_db(&app_state.db, data.into()).await.map(|res| HttpResponse::Ok().json(res))
}

pub async fn update_photo_category(
    app_state: web::Data<AppState>,
    data: web::Json<UpdateCategory>,
) -> Result<HttpResponse, MyError> {
    update_photo_category_db(&app_state.db, data.into()).await.map(|res| HttpResponse::Ok().json(res))
}

pub async fn delete_photo_category(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let id = params.into_inner();
    del_photo_category_db(&app_state.db, id).await.map(|res| HttpResponse::Ok().json(res))
}
