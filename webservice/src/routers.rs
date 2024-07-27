use crate::handlers::common::*;
use crate::handlers::general::*;
use actix_web::web;
use crate::handlers::about::*;

pub fn health_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/health")
            .route("", web::get().to(health_check_handler))
    );
}

pub fn web_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/web").service(
            web::scope("/about")
                .route("", web::get().to(get_about))
        )
    );
}

pub fn admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/login")
                .route("", web::post().to(valid_login))
            )
            .service(
                web::scope("/about")
                    .route("", web::get().to(get_about))
                    .route("", web::put().to(update_about)),
            )
    );
}
