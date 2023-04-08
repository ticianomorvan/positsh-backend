use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Debug, Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct User {
    pub email: String,
    pub fullname: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoginInformation {
    pub email: String,
    pub password: String,
}
