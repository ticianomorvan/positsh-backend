use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{errors::CustomError, models::Posit};

pub async fn create_posit(client: &Client, posit_body: Posit) -> Result<Posit, CustomError> {
    let _statement = include_str!("../sql/create_posit.sql");
    let _statement = _statement.replace("$table_fields", &Posit::sql_table_fields());
    let statement = client.prepare(&_statement).await.unwrap();

    client
        .query(
            &statement,
            &[
                &posit_body.title,
                &posit_body.content,
                &posit_body.topic,
                &posit_body.author_name,
            ],
        )
        .await?
        .iter()
        .map(|row| Posit::from_row_ref(row).unwrap())
        .collect::<Vec<Posit>>()
        .pop()
        .ok_or(CustomError::NotFound)
}
