use crate::errors::MyError;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use crate::dbaccess::photo_tv_series::*;
use crate::models::photo::{CreatTvSeries, UpdateTvSeries};

pub async fn get_photo_tv_series(
    app_state: web::Data<AppState>,
    data: web::Json<CreatTvSeries>,
) -> Result<HttpResponse, MyError> {
    get_photo_tv_series_db(&app_state.db, data.into()).await.map(|res| HttpResponse::Ok().json(res))
}

pub async fn get_photo_tv_series_details(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let id = params.into_inner();
    get_photo_tv_series_details_db(&app_state.db, id).await.map(|res| HttpResponse::Ok().json(res))
}

pub async fn add_photo_tv_series(
    app_state: web::Data<AppState>,
    data: web::Json<CreatTvSeries>,
) -> Result<HttpResponse, MyError> {
    add_photo_tv_series_db(&app_state.db, data.into()).await.map(|res| HttpResponse::Ok().json(res))
}

pub async fn update_photo_tv_series(
    app_state: web::Data<AppState>,
    data: web::Json<UpdateTvSeries>,
) -> Result<HttpResponse, MyError> {
    update_photo_tv_series_db(&app_state.db, data.into()).await.map(|res| HttpResponse::Ok().json(res))
}

pub async fn delete_photo_tv_series(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let id = params.into_inner();
    del_photo_tv_series_db(&app_state.db, id).await.map(|res| HttpResponse::Ok().json(res))
}
