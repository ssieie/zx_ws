use crate::errors::MyError;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use crate::dbaccess::photo::*;
use crate::models::photo::{CreatPhoto, UpdatePhoto};
use crate::models::common::Pager;

pub async fn get_photo_list(
    app_state: web::Data<AppState>,
    data: web::Json<CreatPhoto>,
) -> Result<HttpResponse, MyError> {
    get_photo_list_db(&app_state.db, data.into()).await.map(|res| HttpResponse::Ok().json(res))
}

pub async fn get_photo_hot_list(
    app_state: web::Data<AppState>,
    page:web::Json<Pager>
) -> Result<HttpResponse, MyError> {
    get_photo_list_hot_db(&app_state.db,page.into()).await.map(|res| HttpResponse::Ok().json(res))
}

pub async fn get_photo_new_list(
    app_state: web::Data<AppState>,
    page:web::Json<Pager>
) -> Result<HttpResponse, MyError> {
    get_photo_list_new_db(&app_state.db,page.into()).await.map(|res| HttpResponse::Ok().json(res))
}

pub async fn get_photo(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let id = params.into_inner();
    get_photo_details_db(&app_state.db, id).await.map(|res| HttpResponse::Ok().json(res))
}

pub async fn add_photo(
    app_state: web::Data<AppState>,
    data: web::Json<CreatPhoto>,
) -> Result<HttpResponse, MyError> {
    add_photo_db(&app_state.db, data.into()).await.map(|res| HttpResponse::Ok().json(res))
}

pub async fn update_photo(
    app_state: web::Data<AppState>,
    data: web::Json<UpdatePhoto>,
) -> Result<HttpResponse, MyError> {
    update_photo_db(&app_state.db, data.into()).await.map(|res| HttpResponse::Ok().json(res))
}

pub async fn delete_photo(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let id = params.into_inner();
    del_photo_db(&app_state.db, id).await.map(|res| HttpResponse::Ok().json(res))
}
