use crate::handlers::common::*;
use crate::handlers::general::*;
use actix_web::web;

pub fn web_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/login")
            .route("", web::post().to(valid_login))
    );
}
