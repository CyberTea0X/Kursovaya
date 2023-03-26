use std::fs;

use crate::{
    database::{self, DBconfig},
    email, passwords,
};
use actix_web::{post, web, Responder, Result as ActxResult};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
pub struct RegisterInfo {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub about: Option<String>,
    pub age: Option<String>,
    pub gender: Option<String>,
}

#[post("/register/{username}/{email}/{password}")]
pub async fn register_user_service(
    path: web::Path<(String, String, String)>,
    db_config: web::Data<DBconfig>,
    info: web::Query<RegisterInfo>,
) -> ActxResult<impl Responder> {
    let (username, email, password) = path.into_inner();
    let (status, fail_reason) = (|| {
        if !email::is_valid_email(email.as_str()) {
            return ("FAILED".to_owned(), "Invalid email adresss".to_owned());
        }
        if !passwords::is_valid_password(&password) {
            return (
                "FAILED".to_owned(),
                "Password contains invalid characters or too small".to_owned(),
            );
        }
        let connection = database::try_connect(&db_config, 3);
        if connection.is_err() {
            println!("Failed to connect to database");
            return (
                "FAILED".to_owned(),
                "Failed to connect to database".to_owned(),
            );
        }
        let mut connection = connection.unwrap();
        match database::user_exists(&mut connection, &email) {
            Ok(true) => return ("FAILED".to_owned(), "User already registered".to_owned()),
            Err(_) => return ("FAILED".to_owned(), "Database error".to_owned()),
            Ok(false) => ()
        }
        if let Err(err) = database::register_user(&mut connection, &username, &email, &password, &info) {
            println!("{:?}", err);
            let err_content = err.to_string();
            if err_content.contains("ERROR 1406") {
                return (
                    "FAILED".to_owned(),
                    // MySqlError { ERROR 1406 (22001): Data too long for column 'firstname' at row 1 }
                    err_content[err_content.find("Data").unwrap()
                        ..err_content.rfind(" at row 1").unwrap()]
                        .to_owned(),
                );
            }
            return ("FAILED".to_owned(), "Database error".to_owned());
        }
        let id = match database::user_email_to_id(&mut connection, &email) {
            Ok(Some(id)) => id,
            _ => return ("FAILED".to_owned(), "Internal server error".to_owned())
        };
        let dir_name = format!("users/{}", id);
        if fs::create_dir(dir_name).is_err() {
            return ("FAILED".to_owned(), "Internal server error".to_owned());
        }
        let dir_name = format!("users/{}/gallery", id);
        if fs::create_dir(dir_name).is_err() {
            return ("FAILED".to_owned(), "Internal server error".to_owned());
        }
        return ("OK".to_owned(), "".to_owned())
    })();
    Ok(web::Json(json!({
        "status": status,
        "reason": fail_reason,
    })))
}
