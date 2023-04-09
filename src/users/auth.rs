use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::{bearer, AuthenticationError};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::Error as JWTError, DecodingKey, EncodingKey, Header, TokenData,
    Validation,
};
use uuid::Uuid;

use crate::utilities::schemas::JWTClaims;

pub fn create_jwt(user_id: Uuid, email: String) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(7))
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

pub fn validate_token(token: &str) -> Result<TokenData<JWTClaims>, JWTError> {
    let secret = std::env::var("JWT_SECRET").unwrap();

    decode::<JWTClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(jsonwebtoken::Algorithm::HS512),
    )
}

pub async fn validator(
    request: ServiceRequest,
    credentials: bearer::BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let config = request
        .app_data::<bearer::Config>()
        .cloned()
        .unwrap_or_default();

    match validate_token(credentials.token()) {
        Ok(result) => {
            request.extensions_mut().insert(result.claims);
            Ok(request)
        }
        Err(_) => Err((AuthenticationError::from(config).into(), request)),
    }
}
