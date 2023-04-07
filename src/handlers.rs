use actix_web::{delete, get, patch, post, web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

use crate::{
    database,
    errors::CustomError,
    models::{PartialPosit, Posit},
};

#[get("")]
pub async fn get_posits(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = pool.get().await.map_err(CustomError::PoolError)?;
    let posits = database::get_posits(&client).await?;

    Ok(HttpResponse::Ok().json(posits))
}

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

#[get("/{id}")]
pub async fn get_posit(
    id: web::Path<String>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = pool.get().await.map_err(CustomError::PoolError)?;
    let posit = database::get_posit(&client, id.to_string()).await?;

    Ok(HttpResponse::Ok().json(posit))
}

#[patch("/{id}")]
pub async fn update_posit(
    id: web::Path<String>,
    update_body: web::Json<PartialPosit>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let new_posit_body: PartialPosit = update_body.into_inner();
    let client = pool.get().await.map_err(CustomError::PoolError)?;
    let updated_posit = database::update_posit(&client, id.to_string(), new_posit_body).await?;

    Ok(HttpResponse::Ok().json(updated_posit))
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
