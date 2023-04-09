use serde::{Deserialize, Serialize};

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
