use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct JWTClaims {
    pub sub: Uuid,
    pub user: String,
    pub exp: usize,
}
