use std::fs;
use std::fs::File;
use actix_multipart::{
    form::{
        tempfile::{TempFile},
        MultipartForm,
    },
};
use actix_web::HttpResponse;
use chrono::Local;
use log::{info, warn};
use mime_guess::from_path;
use crate::common::api_response::ApiResponse;
use crate::errors::MyError;
use crate::config::config::{DEVELOPMENT_BUCKET_URL, PRODUCTION_BUCKET_URL};
use serde::Serialize;
use std::io;
use std::io::BufReader;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use image::{DynamicImage, ImageFormat};
use image::codecs::jpeg::JpegEncoder;

const MAX_SIZE: usize = 100 * 1024 * 1024; // 100MB

const IMAGE_QUALITY: u8 = 10;

#[derive(Serialize, Debug, Clone)]
pub struct FileInfo {
    url: String,
    name: String,
    #[serde(rename = "compressRename")]
    compress_rename: Option<String>,
    #[serde(rename = "compressUrl")]
    compress_url: Option<String>,
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

            let mut compress_rename: Option<String> = None;
            let mut abs_compress_path: Option<String> = None;

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

            // 获取文件类型, 保存一份图片缩略图
            let mime_type = from_path(&filename).first_or_octet_stream().to_string();
            if mime_type.starts_with("image/") {
                let rename = format!("{}-compress-{}.jpg", timestamp, filename);
                let abs_path = match std::env::consts::OS {
                    "windows" => format!("http://localhost:8080/{}", rename),
                    _ => format!("https://files.zxandhy.top/{}", rename)
                };
                let path = match std::env::consts::OS {
                    "windows" => format!("{}/{}", DEVELOPMENT_BUCKET_URL, rename),
                    _ => format!("{}/{}", PRODUCTION_BUCKET_URL, rename),
                };
                match compress_image(&temp_path, &rename, &mime_type, IMAGE_QUALITY) {
                    Ok(compressed_image_path) => {
                        if let Err(e) = fs::copy(&compressed_image_path, &path) {
                            return Err(MyError::CustomError(e.to_string()));
                        }
                        if let Err(e) = fs::remove_file(&compressed_image_path) {
                            return Err(MyError::CustomError(e.to_string()));
                        }
                        match set_perms(&path) {
                            _ => ()
                        };
                        compress_rename = Some(rename);
                        abs_compress_path = Some(abs_path);
                    }
                    Err(e) => {
                        warn!("图片压缩失败: {}",e);
                        return Err(MyError::CustomError("图片压缩失败".to_string()));
                    }
                }
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
                compress_rename,
                compress_url: abs_compress_path,
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

// 压缩图片
fn compress_image(input_path: &PathBuf, res_path: &str, mime_type: &str, quality: u8) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // 打开图片文件
    let input_file = File::open(input_path)?;
    let input_image: DynamicImage;

    // 明确指定图片格式
    if mime_type.contains("jpeg") || mime_type.contains("jpg") {
        input_image = image::load(BufReader::new(&input_file), ImageFormat::Jpeg)?;
    } else if mime_type.contains("png") {
        input_image = image::load(BufReader::new(&input_file), ImageFormat::Png)?;
    } else if mime_type.contains("bmp") {
        input_image = image::load(BufReader::new(&input_file), ImageFormat::Bmp)?;
    } else if mime_type.contains("gif") {
        input_image = image::load(BufReader::new(&input_file), ImageFormat::Gif)?;
    } else if mime_type.contains("tiff") {
        input_image = image::load(BufReader::new(&input_file), ImageFormat::Tiff)?;
    } else {
        return Err("不支持的图片格式".into());
    }

    let compressed_path = input_path.with_extension(res_path);

    save_compressed_image(&input_image, &compressed_path, quality)?;

    Ok(compressed_path)
}

fn save_compressed_image(image: &DynamicImage, output_path: &PathBuf, quality: u8) -> Result<(), Box<dyn std::error::Error>> {
    let output_file = File::create(output_path)?;

    let mut jpeg_encoder = JpegEncoder::new_with_quality(output_file, quality);

    jpeg_encoder.encode_image(image)?;

    Ok(())
}