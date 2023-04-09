use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreatePositSchema {
    pub title: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    pub user_id: Uuid,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdatePositSchema {
    pub title: Option<String>,
    pub content: Option<String>,
    pub topic: Option<String>,
}
