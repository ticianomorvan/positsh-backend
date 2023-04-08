use actix_web::{post, web, Error, HttpResponse};
use bcrypt::{self, DEFAULT_COST};
use deadpool_postgres::{Client, Pool};

use crate::{
    database,
    errors::CustomError,
    models::user::{LoginInformation, User},
};

fn hash_password(password: String) -> String {
    bcrypt::hash(password, DEFAULT_COST).unwrap()
}

fn verify_password(password: String, hash: &str) -> bool {
    bcrypt::verify(password, hash).unwrap()
}

#[post("")]
pub async fn create_user(
    user: web::Json<User>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let mut user_body: User = user.into_inner();

    user_body = User {
        email: user_body.email,
        fullname: user_body.fullname,
        username: user_body.username,
        password: hash_password(user_body.password),
    };

    let client: Client = pool.get().await.map_err(CustomError::PoolError)?;
    let new_user = database::user::create_user(&client, user_body).await?;

    Ok(HttpResponse::Ok().json(new_user))
}

#[post("/login")]
pub async fn login(
    login: web::Json<LoginInformation>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let login_body: LoginInformation = login.into_inner();
    let client: Client = pool.get().await.map_err(CustomError::PoolError)?;
    let user = database::user::get_user(&client, login_body.email).await?;

    if verify_password(login_body.password, &user.password) {
        Ok(HttpResponse::Ok().json("Bienvenido"))
    } else {
        Ok(HttpResponse::Unauthorized().json("Denegado"))
    }
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(create_user);
    config.service(login);
}
