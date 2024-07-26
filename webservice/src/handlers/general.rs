use actix_web::HttpResponse;

pub async fn health_check_handler() -> HttpResponse {
    let response = format!("{} {} times", 1, 2);
    HttpResponse::Ok().json(&response)
}
