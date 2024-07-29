use actix_web::{web, HttpResponse};
use crate::errors::MyError;
use crate::models::admin::{CreateIntroduce, Introduce};
use crate::state::AppState;
use crate::dbaccess::introduce::*;

pub async fn get_introduce_list(
    app_state: web::Data<AppState>,
    data: web::Json<CreateIntroduce>,
) -> Result<HttpResponse, MyError> {
    get_introduce_list_db(&app_state.db, data.into()).await.map(|res| HttpResponse::Ok().json(res))
}

pub async fn add_introduce(
    app_state: web::Data<AppState>,
    data: web::Json<CreateIntroduce>,
) -> Result<HttpResponse, MyError> {
    add_introduce_db(&app_state.db, data.into()).await.map(|res| HttpResponse::Ok().json(res))
}

pub async fn update_introduce(
    app_state: web::Data<AppState>,
    data: web::Json<Introduce>,
) -> Result<HttpResponse, MyError> {
    update_introduce_db(&app_state.db, data.into()).await.map(|res| HttpResponse::Ok().json(res))
}
pub async fn delete_introduce(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let id = params.into_inner();
    delete_introduce_db(&app_state.db, id).await.map(|res| HttpResponse::Ok().json(res))
}