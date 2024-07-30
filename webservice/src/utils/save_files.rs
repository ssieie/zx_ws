use actix_multipart::{
    form::{
        tempfile::{TempFile},
        MultipartForm,
    },
};
use actix_web::HttpResponse;
use log::info;
use crate::common::api_response::ApiResponse;
use crate::errors::MyError;

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file", limit = "100MB")]
    files: Vec<TempFile>,
}

pub async fn save_files(MultipartForm(form): MultipartForm<UploadForm>) -> Result<HttpResponse, MyError> {
    for f in form.files {
        if let Some(filename) = f.file_name {
            let path = format!("./bucket/{}", filename);
            info!("保存到 {path}");

            let temp_path = f.file.path().to_path_buf();

            match f.file.persist(path.clone()) {
                Ok(_) => info!("文件成功保存到 {}",temp_path.to_str().unwrap_or_else(||"Unknown")),
                Err(_) => {

                    if let Err(e) = std::fs::copy(&temp_path, &path) {
                        return Err(MyError::CustomError(e.to_string()));
                    }
                    if let Err(e) = std::fs::remove_file(&temp_path) {
                        return Err(MyError::CustomError(e.to_string()));
                    }

                    info!("跨驱动器：文件成功复制到 {path}，并删除了临时文件");
                }
            }
        } else {
            return Err(MyError::CustomError("文件名不能为空".into()));
        }
    }

    Ok(HttpResponse::Ok().json(ApiResponse::success("", "上传成功")))
}