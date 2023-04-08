use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreatePositSchema {
    pub title: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdatePositSchema {
    pub title: Option<String>,
    pub content: Option<String>,
    pub topic: Option<String>,
}

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
