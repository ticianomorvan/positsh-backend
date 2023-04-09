use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;

use crate::{
    users::auth::create_jwt,
    users::model::{ReducedUserModel, UserModel},
    users::schema::{CreateUserSchema, LoginRequestSchema},
    AppState,
};

#[get("/{id}")]
pub async fn fetch_user(path: web::Path<Uuid>, data: web::Data<AppState>) -> impl Responder {
    let id: Uuid = path.into_inner();
    let result = sqlx::query_as!(
        ReducedUserModel,
        "SELECT id, username, fullname, email FROM users WHERE id = $1",
        id
    )
    .fetch_one(&data.database)
    .await;

    match result {
        Ok(user) => {
            return HttpResponse::Ok().json(json!({
                "status": "success", "data": json!({ "user": user })
            }))
        }
        Err(_) => {
            return HttpResponse::NotFound().json(json!({
                "status": "error", "message": "No user found with that ID."
            }))
        }
    }
}

#[post("")]
pub async fn create_user(
    body: web::Json<CreateUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let hashed_password = bcrypt::hash(body.password.clone(), bcrypt::DEFAULT_COST)
        .expect("There was an error hashing the password.");

    let result = sqlx::query_as!(
      UserModel,
      "INSERT INTO users (username, fullname, password, email) VALUES ($1, $2, $3, $4) RETURNING *",
      body.username.to_string(),
      body.fullname.to_string(),
      hashed_password,
      body.email.to_string()
    )
    .fetch_one(&data.database)
    .await;

    match result {
        Ok(user) => {
            return HttpResponse::Ok().json(json!({
              "status": "success", "data": json!({ "user": user })
            }))
        }
        Err(error) => {
            if error.to_string().contains("duplicate key value") {
                return HttpResponse::InternalServerError().json(json!({
                    "status": "error", "message": "Username or email already exists."
                }));
            }

            return HttpResponse::InternalServerError().json(json!({
              "status": "error", "message": "There was an unexpected error."
            }));
        }
    }
}

#[post("/login")]
pub async fn login(
    body: web::Json<LoginRequestSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let result = sqlx::query_as!(
        UserModel,
        "SELECT * FROM users WHERE email = $1",
        body.email
    )
    .fetch_one(&data.database)
    .await;

    match result {
        Ok(user) => {
            let passwords_match = bcrypt::verify(body.password.clone(), &user.password).unwrap();

            if passwords_match {
                let token = create_jwt(user.id.clone(), user.username.clone());
                return HttpResponse::Ok().json(json!({
                  "status": "success", "data": json!({ "user": user, "token": token })
                }));
            } else {
                return HttpResponse::BadRequest().json(json!({
                  "status": "error", "message": "Credentials provided doesn't match."
                }));
            }
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json(json!({
              "status": "error", "message": "Failed to fetch user."
            }))
        }
    }
}

pub fn init_handler(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/users")
            .service(create_user)
            .service(fetch_user)
            .service(login),
    );
}
