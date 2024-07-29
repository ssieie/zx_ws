use crate::errors::MyError;
use crate::models::login::Login;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use crate::dbaccess::common::{valid_login_db,request_log_db};
use crate::models::request_log::RequestLogParams;

pub async fn valid_login(
    app_state: web::Data<AppState>,
    login: web::Json<Login>,
) -> Result<HttpResponse, MyError> {
    valid_login_db(app_state,login.try_into()?).await.map(|res| HttpResponse::Ok().json(res))
}

pub async fn request_log(
    app_state: web::Data<AppState>,
    data: web::Json<RequestLogParams>,
) -> Result<HttpResponse, MyError> {
    request_log_db(&app_state.db,data.into()).await.map(|res| HttpResponse::Ok().json(res))
}
