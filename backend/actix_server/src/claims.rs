use actix_web::error::ErrorUnauthorized;
use actix_web::Error;
use chrono::{Duration, Utc};
use jsonwebtoken::{self, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct JwtConfig {
    pub expiration_hours: i64,
    pub secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    pub email: String,
    pub permissions: Vec<String>,
    pub exp: i64,
}

impl Claims {
    pub fn new(email: String, permissions: Vec<String>, jwt_config: &JwtConfig) -> Self {
        Self {
            email,
            permissions,
            exp: (Utc::now() + Duration::hours(jwt_config.expiration_hours)).timestamp(),
        }
    }
}

/// Create a json web token (JWT)
pub(crate) fn create_jwt(claims: Claims, jwt_config: &JwtConfig) -> Result<String, Error> {
    let encoding_key = EncodingKey::from_secret(jwt_config.secret.as_bytes());
    jsonwebtoken::encode(&Header::default(), &claims, &encoding_key)
        .map_err(|e| ErrorUnauthorized(e.to_string()))
}

/// Decode a json web token (JWT)
pub(crate) fn decode_jwt(token: &str, jwt_config: &JwtConfig) -> Result<Claims, Error> {
    let decoding_key = DecodingKey::from_secret(jwt_config.secret.as_bytes());
    jsonwebtoken::decode::<Claims>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| ErrorUnauthorized(e.to_string()))
}
