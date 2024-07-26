use crate::errors::MyError;
use crate::models::login::Login;
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn valid_login(
    login: web::Json<Login>,
    _app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    // post_new_course_db(&app_state.db, login)
    //     .await
    //     .map(|course| HttpResponse::Ok().json(course))

    Ok(HttpResponse::Ok().json(login))
}
