mod handler;
mod model;
mod schema;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::{get, http::header, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub struct AppState {
    database: Pool<Postgres>,
}

#[get("/api/health")]
async fn health() -> impl Responder {
    const MESSAGE: &str = "SERVER IS RUNNING";

    HttpResponse::Ok().json(json!({"status": "active", "message": MESSAGE}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info")
    }

    dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connected to PostgreSQL.");
            pool
        }
        Err(error) => {
            println!("Error connecting to PostgreSQL: {:?}", error);
            std::process::exit(1);
        }
    };

    println!("Server successfuly started.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                database: pool.clone(),
            }))
            .configure(handler::init_handler)
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
                    .allowed_headers(vec![
                        header::CONTENT_TYPE,
                        header::AUTHORIZATION,
                        header::ACCEPT,
                    ])
                    .supports_credentials(),
            )
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
