use actix_web::{delete, get, post, web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

use crate::{database, errors::CustomError, models::Posit};

#[post("")]
pub async fn create_posit(
    posit: web::Json<Posit>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let posit_body: Posit = posit.into_inner();
    let client: Client = pool.get().await.map_err(CustomError::PoolError)?;
    let new_posit = database::create_posit(&client, posit_body).await?;

    Ok(HttpResponse::Ok().json(new_posit))
}

#[get("")]
pub async fn get_posits(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = pool.get().await.map_err(CustomError::PoolError)?;
    let posits = database::get_posits(&client).await?;

    Ok(HttpResponse::Ok().json(posits))
}

#[delete("/{id}")]
pub async fn delete_posit(
    id: web::Path<String>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = pool.get().await.map_err(CustomError::PoolError)?;
    let result = database::delete_posit(&client, id.to_string()).await?;

    Ok(HttpResponse::Ok().json(result))
}
