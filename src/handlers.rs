use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

use crate::{database, errors::CustomError, models::Posit};

pub async fn create_posit(
    posit: web::Json<Posit>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let posit_body: Posit = posit.into_inner();
    let client: Client = pool.get().await.map_err(CustomError::PoolError)?;
    let new_posit = database::create_posit(&client, posit_body).await?;

    Ok(HttpResponse::Ok().json(new_posit))
}
