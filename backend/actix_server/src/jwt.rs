// use serde::Deserialize;
// use crate::claims::Claims;

// #[derive(Deserialize)]
// pub struct UserPermissions {
//     pub username: String,
//     pub permissions: Vec<String>,
// }

// pub fn create_token(info: web::Json<UserPermissions>) -> Result<String, Error> {
//     let user_info = info.into_inner();
//     // Create a JWT
//     let claims = Claims::new(user_info.username, user_info.permissions);
//     let jwt = claims::create_jwt(claims)?;

//     // Return token for work with example handlers
//     Ok(jwt)
// }