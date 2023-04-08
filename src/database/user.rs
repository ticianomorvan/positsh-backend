use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
// use uuid::Uuid;

use crate::{errors::CustomError, models::user::User};

pub async fn create_user(client: &Client, user_body: User) -> Result<User, CustomError> {
    let _statement = include_str!("../../sql/user/create_user.sql");
    let _statement = _statement.replace("$table_fields", &User::sql_table_fields());
    let statement = client.prepare(&_statement).await.unwrap();

    client
        .query(
            &statement,
            &[
                &user_body.fullname,
                &user_body.username,
                &user_body.password,
                &user_body.email,
            ],
        )
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(CustomError::NotFound)
}

pub async fn get_user(client: &Client, email: String) -> Result<User, CustomError> {
    let _statement = include_str!("../../sql/user/get_user.sql");
    let statement = client.prepare(&_statement).await.unwrap();

    let row = client.query_one(&statement, &[&email]).await?;
    let user: User = User::from_row_ref(&row).unwrap();

    Ok(user)
}
