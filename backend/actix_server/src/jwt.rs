use crate::claims::JwtConfig;
use crate::claims::{create_jwt, Claims};
use actix_web::Error;
use serde::Deserialize;

pub fn create_token(
    email: &str,
    permissions: Vec<String>,
    jwt_config: &JwtConfig,
) -> Result<String, Error> {
    // Create a JWT
    let claims = Claims::new(email.to_string(), permissions, jwt_config);
    let jwt = create_jwt(claims, jwt_config)?;

    // Return token for work with example handlers
    Ok(jwt)
}
