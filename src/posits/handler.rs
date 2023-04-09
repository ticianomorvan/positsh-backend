use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

use crate::{
    posits::model::PositModel,
    posits::schema::{CreatePositSchema, UpdatePositSchema},
    users,
    utilities::schemas::{FilterOptions, JWTClaims},
    AppState,
};

#[get("")]
pub async fn list_posits(
    options: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = options.limit.unwrap_or(10);
    let offset = (options.page.unwrap_or(1) - 1) * limit;

    let result = sqlx::query_as!(
        PositModel,
        "SELECT * FROM posits ORDER BY id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.database)
    .await;

    match result {
        Ok(posits) => HttpResponse::Ok().json(json!({
          "status": "success", "results": posits.len(), "data": json!({ "posits": posits })
        })),
        Err(_) => HttpResponse::InternalServerError().json(json!({
          "status": "error", "message": "Failed to fetch posits."
        })),
    }
}

#[post("")]
pub async fn create_posit(
    body: web::Json<CreatePositSchema>,
    data: web::Data<AppState>,
    payload: web::ReqData<JWTClaims>,
) -> impl Responder {
    if payload.sub != body.user_id.to_owned() {
        return HttpResponse::Unauthorized().json(json!({
          "status": "error", "message": "You can't create posits for another users."
        }));
    }

    let result = sqlx::query_as!(
        PositModel,
        "INSERT INTO posits (title, content, topic, user_id) VALUES ($1, $2, $3, $4) RETURNING *",
        body.title.to_string(),
        body.content.to_string(),
        body.topic.to_owned().unwrap_or("".to_string()),
        body.user_id.to_owned()
    )
    .fetch_one(&data.database)
    .await;

    match result {
        Ok(posit) => {
            return HttpResponse::Ok().json(json!({
              "status": "success", "data": json!({ "posit": posit })
            }));
        }
        Err(error) => {
            if error.to_string().contains("duplicate key") {
                return HttpResponse::BadRequest().json(json!({
                  "status": "error", "message": "Posit with that title already exists."
                }));
            }

            return HttpResponse::InternalServerError().json(json!({
              "status": "error", "message": "Failed to get indicated posit"
            }));
        }
    }
}

#[get("/{id}")]
async fn fetch_posit(path: web::Path<Uuid>, data: web::Data<AppState>) -> impl Responder {
    let id: Uuid = path.into_inner();

    let result = sqlx::query_as!(PositModel, "SELECT * FROM posits WHERE id = $1", id)
        .fetch_one(&data.database)
        .await;

    match result {
        Ok(posit) => HttpResponse::Ok().json(json!({
          "status": "success", "data": json!({ "posit": posit })
        })),
        Err(_) => HttpResponse::NotFound().json(json!({
          "status": "error", "message": "Posit not found."
        })),
    }
}

#[patch("/{id}")]
pub async fn update_posit(
    path: web::Path<Uuid>,
    body: web::Json<UpdatePositSchema>,
    data: web::Data<AppState>,
    payload: web::ReqData<JWTClaims>,
) -> impl Responder {
    let id: Uuid = path.into_inner();
    let result = sqlx::query_as!(PositModel, "SELECT * FROM posits WHERE id = $1", id)
        .fetch_one(&data.database)
        .await;

    if result.is_err() {
        return HttpResponse::InternalServerError().json(json!({
          "status": "error", "message": "Failed to fetch indicated posit."
        }));
    }

    let posit = result.unwrap();

    if posit.user_id != payload.sub {
        return HttpResponse::Unauthorized().json(json!({
          "status": "error", "message": "You aren't authorized to update this posit."
        }));
    }

    let result = sqlx::query_as!(
      PositModel,
      "UPDATE posits SET title = $1, content = $2, topic = $3, updated_at = $4 WHERE id = $5 RETURNING *",
      body.title.to_owned().unwrap_or(posit.title),
      body.content.to_owned().unwrap_or(posit.content),
      body.topic.to_owned().unwrap_or(posit.topic.unwrap()),
      Utc::now(),
      id
    )
    .fetch_one(&data.database)
    .await;

    match result {
        Ok(posit) => HttpResponse::Ok().json(json!({
          "status": "success", "data": json!({ "posit": posit })
        })),
        Err(_) => HttpResponse::InternalServerError().json(json!({
          "status": "error", "message": "Failed to update indicated posit."
        })),
    }
}

#[delete("/{id}")]
pub async fn delete_posit(
    path: web::Path<Uuid>,
    data: web::Data<AppState>,
    payload: web::ReqData<JWTClaims>,
) -> impl Responder {
    let id: Uuid = path.into_inner();

    let result = sqlx::query_as!(PositModel, "SELECT * FROM posits WHERE id = $1", id)
        .fetch_one(&data.database)
        .await;

    if result.is_err() {
        return HttpResponse::InternalServerError().json(json!({
          "status": "error", "message": "Failed to fetch indicated posit."
        }));
    }

    if result.unwrap().user_id != payload.sub {
        return HttpResponse::Unauthorized().json(json!({
          "status": "error", "message": "You aren't authorized to delete this posit."
        }));
    }

    let rows_affected = sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(&data.database)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        return HttpResponse::InternalServerError().json(json!({
          "status": "error", "message": "Posit couldn't be deleted."
        }));
    }

    HttpResponse::Ok().finish()
}

pub fn init_handler(config: &mut web::ServiceConfig) {
    let jwt_middleware = HttpAuthentication::bearer(users::auth::validator);

    config.service(
        web::scope("/api/posits")
            .wrap(jwt_middleware)
            .service(list_posits)
            .service(create_posit)
            .service(fetch_posit)
            .service(update_posit)
            .service(delete_posit),
    );
}
