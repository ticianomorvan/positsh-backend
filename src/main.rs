mod posits;
mod users;
mod utilities;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::{http::header, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub struct AppState {
    database: Pool<Postgres>,
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
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173")
                    .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
                    .allowed_headers(vec![
                        header::CONTENT_TYPE,
                        header::AUTHORIZATION,
                        header::ACCEPT,
                    ])
                    .supports_credentials(),
            )
            .wrap(Logger::default())
            .configure(posits::handler::init_handler)
            .configure(users::handler::init_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
