use crate::handlers::common::*;
use crate::handlers::general::*;
use actix_web::web;
use crate::handlers::about::*;
use crate::handlers::article::*;
use crate::handlers::category::*;
use crate::handlers::introduce::*;
use crate::utils::save_files::save_files;

pub fn health_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/health")
            .route("", web::get().to(health_check_handler))
    );
}

pub fn web_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/web")
            .service(
                web::scope("/about")
                    .route("", web::get().to(get_about))
            )
            .service(
                web::scope("/article")
                    .route("", web::get().to(get_article_list_web))
                    .route("/all", web::get().to(get_article_list_all_web))
                    .route("/hot", web::post().to(get_article_hot_list_web))
                    .route("/{id:\\d+}", web::get().to(get_article))
            )
            .service(
                web::scope("/introduce")
                    .route("", web::post().to(get_introduce_list))
            )
            .service(
                web::scope("/category")
                    .route("/hot", web::post().to(get_hot_category_list))
            )
    );
}

pub fn admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/requestLog")
                .route("", web::post().to(request_log))
            )
            .service(web::scope("/login")
                .route("", web::post().to(valid_login))
            )
            .service(web::scope("/upload")
                .route("", web::post().to(save_files))
            )
            .service(
                web::scope("/about")
                    .route("", web::get().to(get_about))
                    .route("", web::put().to(update_about)),
            )
            .service(
                web::scope("/category")
                    .route("/list", web::post().to(get_category))
                    .route("/add", web::post().to(add_category))
                    .route("/update", web::post().to(update_category))
                    .route("/{id}", web::delete().to(delete_category)),
            )
            .service(
                web::scope("/article")
                    .route("/list", web::post().to(get_article_list))
                    .route("/add", web::post().to(add_article))
                    .route("/update", web::post().to(update_article))
                    .route("/{id}", web::get().to(get_article))
                    .route("/{id}", web::delete().to(delete_article)),
            )
            .service(
                web::scope("/introduce")
                    .route("/list", web::post().to(get_introduce_list))
                    .route("/add", web::post().to(add_introduce))
                    .route("/update", web::post().to(update_introduce))
                    .route("/{id}", web::delete().to(delete_introduce)),
            )
    );
}
