use std::fs;
use actix_multipart::{
    form::{
        tempfile::{TempFile},
        MultipartForm,
    },
};
use actix_web::HttpResponse;
use chrono::Local;
use log::info;
use crate::common::api_response::ApiResponse;
use crate::errors::MyError;
use crate::config::config::{DEVELOPMENT_BUCKET_URL, PRODUCTION_BUCKET_URL};
use serde::Serialize;
use std::io;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

const MAX_SIZE: usize = 100 * 1024 * 1024; // 100MB

#[derive(Serialize, Debug, Clone)]
pub struct FileInfo {
    url: String,
    name: String,
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file", limit = "100MB")]
    files: Vec<TempFile>,
}

pub async fn save_files(MultipartForm(form): MultipartForm<UploadForm>) -> Result<HttpResponse, MyError> {
    let mut file_res: Vec<FileInfo> = vec![];

    for f in form.files {
        if let Some(filename) = f.file_name {
            let timestamp = Local::now().timestamp();

            let rename = format!("{}-{}", timestamp, filename);

            let path = match std::env::consts::OS {
                "windows" => format!("{}/{}", DEVELOPMENT_BUCKET_URL, rename),
                _ => format!("{}/{}", PRODUCTION_BUCKET_URL, rename),
            };

            let abs_path = match std::env::consts::OS {
                "windows" => format!("http://localhost:8080/{}", rename),
                _ => format!("https://files.zxandhy.top/{}", rename)
            };

            let temp_path = f.file.path().to_path_buf();

            if f.size == 0 {
                return Err(MyError::CustomError("文件大小获取失败".into()));
            }

            if f.size > MAX_SIZE {
                return Err(MyError::CustomError("文件大小超出限制".into()));
            }

            match f.file.persist(path.clone()) {
                Ok(_) => {
                    match set_perms(&path) {
                        _ => ()
                    };
                    info!("文件成功保存到 {}",path)
                }
                Err(_) => {
                    if let Err(e) = fs::copy(&temp_path, &path) {
                        return Err(MyError::CustomError(e.to_string()));
                    }
                    if let Err(e) = fs::remove_file(&temp_path) {
                        return Err(MyError::CustomError(e.to_string()));
                    }

                    info!("跨驱动器：文件成功复制到 {path}，并删除了临时文件");

                    match set_perms(&path) {
                        _ => ()
                    };
                }
            }

            file_res.push(FileInfo {
                name: rename,
                url: abs_path,
            });
        } else {
            return Err(MyError::CustomError("文件名不能为空".into()));
        }
    }

    Ok(HttpResponse::Ok().json(ApiResponse::success(file_res, "上传成功")))
}

fn set_perms(path: &str) -> io::Result<()> {
    #[allow(unused)]
    let metadata = fs::metadata(path)?;

    #[cfg(unix)]
    {
        // 在 Unix 平台上设置权限
        let mut perms = metadata.permissions();
        perms.set_mode(0o644);
        fs::set_permissions(path, perms)?;
    }

    #[cfg(windows)]
    {}

    Ok(())
}