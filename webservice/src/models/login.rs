use serde::{Deserialize, Serialize};
use actix_web::web;
use crate::errors::MyError;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Login {
    pub username: String,
    pub password: String,
}

impl TryFrom<web::Json<Login>> for Login {
    type Error = MyError;
    fn try_from(login: web::Json<Login>) -> Result<Self, Self::Error> {
        Ok(Login {
            username: login.username.clone(),
            password: login.password.clone(),
        })
    }
}
