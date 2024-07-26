use crate::errors::MyError;
use crate::models::login::Login;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use crate::dbaccess::common::valid_login_db;

pub async fn valid_login(
    app_state: web::Data<AppState>,
    login: web::Json<Login>,
) -> Result<HttpResponse, MyError> {
    valid_login_db(&app_state.db, login.try_into()?).await.map(|res| HttpResponse::Ok().json(res))
}
