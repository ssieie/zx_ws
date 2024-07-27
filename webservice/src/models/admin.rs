use serde::{Deserialize, Serialize};
use actix_web::web;
use crate::errors::MyError;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct About {
    pub content: String,
}

impl TryFrom<web::Json<About>> for About {
    type Error = MyError;
    fn try_from(about: web::Json<About>) -> Result<Self, Self::Error> {
        Ok(About {
            content: about.content.clone(),
        })
    }
}
