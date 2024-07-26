use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize, Debug, Clone)]
pub struct Login {
    pub username: String,
    pub password: String,
}
