use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct PositModel {
    pub id: Uuid,

    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,

    pub title: String,
    pub content: String,
    pub topic: Option<String>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UserModel {
    pub id: Uuid,

    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,

    pub username: String,
    pub fullname: String,
    pub password: String,
    pub email: String,
}
