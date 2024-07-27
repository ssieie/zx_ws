use crate::errors::MyError;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use crate::dbaccess::about::*;
use crate::models::admin::About;

pub async fn get_about(
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    get_about_db(&app_state.db).await.map(|res| HttpResponse::Ok().json(res))
}


pub async fn update_about(
    app_state: web::Data<AppState>,
    about: web::Json<About>,
) -> Result<HttpResponse, MyError> {
    update_about_db(&app_state.db, about.try_into()?).await.map(|res| HttpResponse::Ok().json(res))
}
