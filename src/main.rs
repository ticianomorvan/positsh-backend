mod config;
mod database;
mod errors;
mod handlers;
mod models;

use ::config::Config;
use actix_cors::Cors;
use actix_web::{http::header, middleware, web, App, HttpServer};
use dotenv::dotenv;
use tokio_postgres::NoTls;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: config::AppConfig = config.try_deserialize().unwrap();

    let pool = config.postgres.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173")
                    .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
                    .allowed_headers(vec![
                        header::ACCEPT,
                        header::AUTHORIZATION,
                        header::CONTENT_TYPE,
                    ])
                    .max_age(3600),
            )
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/posits").configure(handlers::posit::init_routes))
            .service(web::scope("/users").configure(handlers::user::init_routes))
    })
    .bind(config.server_address.clone())?
    .run();

    println!("Server running at http://{}/", config.server_address);

    server.await
}
