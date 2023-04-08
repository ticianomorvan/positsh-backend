use crate::{
    model::PositModel,
    schema::{CreatePositSchema, FilterOptions},
    AppState,
};
use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;

#[get("/posits")]
pub async fn list_posits(
    options: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = options.limit.unwrap_or(10);
    let offset = (options.page.unwrap_or(1) - 1) * limit;

    let result = sqlx::query_as!(
        PositModel,
        "SELECT * FROM posits ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.database)
    .await;

    if result.is_err() {
        return HttpResponse::InternalServerError().json(json!({
          "status": "error", "message": "Failed to fetch posits."
        }));
    }

    let posits = result.unwrap();
    let response = json!({
      "status": "success",
      "results": posits.len(),
      "posits": posits
    });

    HttpResponse::Ok().json(response)
}

#[post("/posits")]
pub async fn create_posit(
    body: web::Json<CreatePositSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let result = sqlx::query_as!(
        PositModel,
        "INSERT INTO posits (title, content, topic) VALUES ($1, $2, $3) RETURNING *",
        body.title.to_string(),
        body.content.to_string(),
        body.topic.to_owned().unwrap_or("".to_string()),
    )
    .fetch_one(&data.database)
    .await;

    match result {
        Ok(posit) => {
            return HttpResponse::Ok().json(json!({
              "status": "success",
              "data": json!({
                "posit": posit
              })
            }));
        }
        Err(error) => {
            if error.to_string().contains("duplicate key") {
                return HttpResponse::BadRequest().json(json!({
                  "status": "error",
                  "message": "Posit with that title already exists."
                }));
            }

            return HttpResponse::InternalServerError().json(json!({
              "status": "error",
              "message": format!("{:?}", error)
            }));
        }
    }
}

#[get("/posits/{id}")]
async fn fetch_posit(path: web::Path<uuid::Uuid>, data: web::Data<AppState>) -> impl Responder {
    let id: uuid::Uuid = path.into_inner();
    let result = sqlx::query_as!(PositModel, "SELECT * FROM posits WHERE id = $1", id)
        .fetch_one(&data.database)
        .await;

    match result {
        Ok(posit) => HttpResponse::Ok().json(json!({
          "status": "success",
          "data": json!({
            "posit": posit
          })
        })),
        Err(_) => HttpResponse::NotFound().json(json!({
          "status": "error",
          "message": "Posit not found."
        })),
    }
}

pub fn init_handler(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api")
            .service(list_posits)
            .service(create_posit)
            .service(fetch_posit),
    );
}
