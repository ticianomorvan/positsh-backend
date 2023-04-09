use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateUserSchema {
    pub username: String,
    pub fullname: String,
    pub password: String,
    pub email: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateUserSchema {
    pub username: Option<String>,
    pub fullname: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct LoginRequestSchema {
    pub email: String,
    pub password: String,
}
