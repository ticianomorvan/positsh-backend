use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use uuid::Uuid;

use crate::{errors::CustomError, models::Posit};

pub async fn get_posits(client: &Client) -> Result<Vec<Posit>, CustomError> {
    let _statement = include_str!("../sql/get_posits.sql");
    let statement = client.prepare(&_statement).await.unwrap();

    let posits = client
        .query(&statement, &[])
        .await?
        .iter()
        .map(|row| Posit::from_row_ref(row).unwrap())
        .collect::<Vec<Posit>>();

    Ok(posits)
}

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

pub async fn get_posit(client: &Client, id: String) -> Result<Posit, CustomError> {
    let parsed_id: Uuid = Uuid::parse_str(&id).unwrap();
    let _statement = include_str!("../sql/get_posit.sql");
    let statement = client.prepare(&_statement).await.unwrap();

    let row = client.query_one(&statement, &[&parsed_id]).await?;
    let posit = Posit::from_row_ref(&row).unwrap();

    Ok(posit)
}

pub async fn delete_posit(client: &Client, id: String) -> Result<u64, CustomError> {
    let parsed_id: Uuid = Uuid::parse_str(&id).unwrap();
    let _statement = include_str!("../sql/delete_posit.sql");
    let statement = client.prepare(&_statement).await.unwrap();

    let result = client.execute(&statement, &[&parsed_id]).await?;

    Ok(result)
}
