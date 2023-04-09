use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;

use crate::{
    users::model::UserModel,
    users::schema::CreateUserSchema,
    utilities::schemas::{FilterOptions, JWTClaims},
    AppState,
};

use super::schema::LoginRequestSchema;

#[get("/users")]
pub async fn list_users(
    options: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = options.limit.unwrap_or(10);
    let offset = (options.page.unwrap_or(1) - 1) * limit;

    let result = sqlx::query_as!(
        UserModel,
        "SELECT * FROM USERS ORDER BY id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32,
    )
    .fetch_all(&data.database)
    .await;

    if result.is_err() {
        return HttpResponse::InternalServerError().json(json!({
          "status": "errir",
          "message": "Failed to fetch users."
        }));
    }

    let users = result.unwrap();

    HttpResponse::Ok().json(json!({
      "status": "success",
      "results": users.len(),
      "users": users
    }))
}

#[post("/users")]
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
              "status": "success",
              "data": json!({
                "user": user
              })
            }))
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(json!({
              "status": "error",
              "message": format!("{:?}", error)
            }))
        }
    }
}

fn create_jwt(user_id: String, email: String) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(60))
        .expect("Valid timestamp")
        .timestamp();

    let claims = JWTClaims {
        sub: user_id,
        user: email,
        exp: expiration as usize,
    };

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET to be set.");

    let header = Header::new(jsonwebtoken::Algorithm::HS512);
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(&secret.as_bytes()),
    )
    .unwrap()
}

#[post("/users/login")]
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
            if bcrypt::verify(body.password.clone(), user.password.as_str()).unwrap() {
                let token = create_jwt(user.id.to_string(), user.username);
                return HttpResponse::Ok().json(json!({
                  "status": "success",
                  "data": json!({
                    "token": token
                  })
                }));
            } else {
                return HttpResponse::BadRequest().json(json!({
                  "status": "error",
                  "message": "Credentials provided doesn't match."
                }));
            }
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(json!({
              "status": "error",
              "message": format!("{:?}", error)
            }))
        }
    }
}

pub fn init_handler(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api")
            .service(list_users)
            .service(create_user)
            .service(login),
    );
}
