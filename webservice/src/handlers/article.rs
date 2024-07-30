use crate::errors::MyError;
use crate::state::AppState;
use chrono::{Utc, Duration};
use actix_web::{web, HttpResponse, HttpRequest};
use crate::dbaccess::article::*;
use crate::models::admin::{CreateArticle, UpdateArticle,ArticleQuery};
use crate::utils::get_real_ip::get_real_ip;

pub async fn get_article_list(
    app_state: web::Data<AppState>,
    data: web::Json<CreateArticle>,
) -> Result<HttpResponse, MyError> {
    get_article_list_db(&app_state.db, data.into()).await.map(|res| HttpResponse::Ok().json(res))
}

pub async fn get_article(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, MyError> {
    let id = params.into_inner();

    let uri = req.uri().to_string();

    // 前台访问才加热度
    if uri.starts_with("/web/article") {

        let ip_address = get_real_ip(&req);

        let mut ip_access_map = app_state.ip_article_access_map.lock().unwrap();
        let now = Utc::now();

        // 检查 IP 和文章 ID 的访问记录
        let access_key = (ip_address.clone(), id);

        if let Some(last_access_time) = ip_access_map.get(&access_key) {
            // 检查上次访问是否超过指定时间间隔
            if *last_access_time + Duration::minutes(30) > now {
                // 未过30分钟
                return get_article_db(&app_state.db, id).await.map(|res| HttpResponse::Ok().json(res));
            }
        }

        // 记录当前访问时间
        ip_access_map.insert(access_key, now);

        // 更新热度计数
        update_article_heat_db(&app_state.db, id).await?;

        get_article_db(&app_state.db, id).await.map(|res| HttpResponse::Ok().json(res))
    }else {
        get_article_db(&app_state.db, id).await.map(|res| HttpResponse::Ok().json(res))
    }
}

pub async fn add_article(
    app_state: web::Data<AppState>,
    data: web::Json<CreateArticle>,
) -> Result<HttpResponse, MyError> {
    add_article_db(&app_state.db, data.into()).await.map(|res| HttpResponse::Ok().json(res))
}

pub async fn update_article(
    app_state: web::Data<AppState>,
    data: web::Json<UpdateArticle>,
) -> Result<HttpResponse, MyError> {
    update_article_db(&app_state.db, data.into()).await.map(|res| HttpResponse::Ok().json(res))
}

pub async fn delete_article(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let id = params.into_inner();
    del_article_db(&app_state.db, id).await.map(|res| HttpResponse::Ok().json(res))
}

//
pub async fn get_article_list_web(
    app_state: web::Data<AppState>
) -> Result<HttpResponse, MyError> {
    get_article_list_web_db(&app_state.db).await.map(|res| HttpResponse::Ok().json(res))
}

pub async fn get_article_list_all_web(
    app_state: web::Data<AppState>,
    query: web::Query<ArticleQuery>
) -> Result<HttpResponse, MyError> {
    get_article_list_all_web_db(&app_state.db,query.cid).await.map(|res| HttpResponse::Ok().json(res))
}

pub async fn get_article_hot_list_web(
    app_state: web::Data<AppState>
) -> Result<HttpResponse, MyError> {
    get_article_hot_list_web_db(&app_state.db).await.map(|res| HttpResponse::Ok().json(res))
}

pub async fn get_hot_category_list(
    app_state: web::Data<AppState>
) -> Result<HttpResponse, MyError> {
    get_hot_category_list_db(&app_state.db).await.map(|res| HttpResponse::Ok().json(res))
}
