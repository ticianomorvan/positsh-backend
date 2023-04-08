use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "posits")]
pub struct Posit {
    pub title: String,
    pub content: String,
    pub topic: String,
    pub author_name: String,
}

#[derive(Deserialize, Serialize)]
pub struct PartialPosit {
    pub title: String,
    pub content: String,
}
