use actix_web::web::Json;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct TvSeries {
    pub id: i32,

    pub name: String,

    #[serde(rename = "coverImage")]
    pub cover_image: Option<String>,

    #[serde(rename = "previewImage")]
    pub preview_image: Option<String>,

    pub author: Option<String>,

    pub describe: Option<String>,

    #[serde(rename = "createTime")]
    pub create_time: Option<NaiveDateTime>,

    #[serde(rename = "updateTime")]
    pub update_time: Option<NaiveDateTime>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreatTvSeries {
    pub name: String,

    #[serde(rename = "coverImage")]
    pub cover_image: Option<String>,

    #[serde(rename = "previewImage")]
    pub preview_image: Option<String>,

    pub author: Option<String>,

    pub describe: Option<String>,
}
impl From<Json<CreatTvSeries>> for CreatTvSeries {
    fn from(tv_series: Json<CreatTvSeries>) -> Self {
        Self {
            name: tv_series.name.clone(),
            cover_image: tv_series.cover_image.clone(),
            preview_image: tv_series.preview_image.clone(),
            author: tv_series.author.clone(),
            describe: tv_series.describe.clone(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateTvSeries {
    pub id: i32,

    pub name: String,

    #[serde(rename = "coverImage")]
    pub cover_image: Option<String>,

    #[serde(rename = "previewImage")]
    pub preview_image: Option<String>,

    pub author: Option<String>,

    pub describe: Option<String>,
}
impl From<Json<UpdateTvSeries>> for UpdateTvSeries {
    fn from(tv_series: Json<UpdateTvSeries>) -> Self {
        Self {
            id: tv_series.id,
            name: tv_series.name.clone(),
            cover_image: tv_series.cover_image.clone(),
            preview_image: tv_series.preview_image.clone(),
            author: tv_series.author.clone(),
            describe: tv_series.describe.clone(),
        }
    }
}

#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Photo {
    pub id: i32,

    pub width: Option<i32>,

    pub height: Option<i32>,

    #[serde(rename = "photoCategoryId")]
    pub photo_category_id: Option<i32>,

    #[serde(rename = "photoCategoryName")]
    pub photo_category_name: Option<String>,

    #[serde(rename = "tvSeriesId")]
    pub tv_series_id: Option<i32>,

    #[serde(rename = "tvSeriesName")]
    pub tv_series_name: Option<String>,

    #[serde(rename = "photoUrl")]
    pub photo_url: String,

    #[serde(rename = "previewUrl")]
    pub preview_url: String,

    pub author: Option<String>,

    pub like: Option<i32>,

    #[serde(rename = "createTime")]
    pub create_time: Option<NaiveDateTime>,

    #[serde(rename = "updateTime")]
    pub update_time: Option<NaiveDateTime>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreatPhoto {
    pub width: Option<i32>,

    pub height: Option<i32>,

    #[serde(rename = "photoCategoryId")]
    pub photo_category_id: Option<i32>,

    #[serde(rename = "tvSeriesId")]
    pub tv_series_id: Option<i32>,

    #[serde(rename = "photoUrl")]
    pub photo_url: String,

    #[serde(rename = "previewUrl")]
    pub preview_url: String,

    pub author: Option<String>,
}
impl From<Json<CreatPhoto>> for CreatPhoto {
    fn from(photo: Json<CreatPhoto>) -> Self {
        Self {
            width: photo.width,
            height: photo.height,
            photo_category_id: photo.photo_category_id.clone(),
            tv_series_id: photo.tv_series_id.clone(),
            photo_url: photo.photo_url.clone(),
            preview_url: photo.preview_url.clone(),
            author: photo.author.clone(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdatePhoto {
    pub id: i32,

    pub width: Option<i32>,

    pub height: Option<i32>,

    #[serde(rename = "photoCategoryId")]
    pub photo_category_id: Option<i32>,

    #[serde(rename = "tvSeriesId")]
    pub tv_series_id: Option<i32>,

    #[serde(rename = "photoUrl")]
    pub photo_url: String,

    #[serde(rename = "previewUrl")]
    pub preview_url: String,

    pub author: Option<String>,
}
impl From<Json<UpdatePhoto>> for UpdatePhoto {
    fn from(photo: Json<UpdatePhoto>) -> Self {
        Self {
            id: photo.id,
            width: photo.width,
            height: photo.height,
            photo_category_id: photo.photo_category_id.clone(),
            tv_series_id: photo.tv_series_id.clone(),
            photo_url: photo.photo_url.clone(),
            preview_url: photo.preview_url.clone(),
            author: photo.author.clone(),
        }
    }
}