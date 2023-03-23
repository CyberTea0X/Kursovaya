use crate::{
    database::{self, DBconfig},
    email, passwords,
};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result as ActxResult};
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
pub async fn register_service(
    path: web::Path<(String, String, String)>,
    db_config: web::Data<DBconfig>,
    info: web::Query<RegisterInfo>,
) -> ActxResult<impl Responder> {
    let (username, email, password) = path.into_inner();
    let (status, fail_reason) = (|| {
        if !email::is_valid_email(email.as_str()) {
            return ("FAILED".to_string(), "Invalid email adresss".to_string());
        }
        if !passwords::is_valid_password(&password) {
            return (
                "FAILED".to_string(),
                "Password contains invalid characters or too small".to_string(),
            );
        }
        let connection = database::try_connect(&db_config, 3);
        if connection.is_err() {
            println!("Failed to connect to database");
            return (
                "FAILED".to_string(),
                "Failed to connect to database".to_string(),
            );
        }
        let mut connection = connection.unwrap();
        if database::user_exists(&mut connection, &email) {
            return ("FAILED".to_string(), "User already registered".to_string());
        }
        match database::register_user(&mut connection, &username, &email, &password, &info) {
            Ok(_) => return ("OK".to_string(), "".to_string()),
            Err(err) => {
                println!("{:?}", err);
                let err_content = err.to_string();
                if err_content.contains("ERROR 1406") {
                    return (
                        "FAILED".to_string(),
                        // MySqlError { ERROR 1406 (22001): Data too long for column 'firstname' at row 1 }
                        err_content[err_content.find("Data").unwrap()
                            ..err_content.rfind(" at row 1").unwrap()]
                            .to_string(),
                    );
                }
                return ("FAILED".to_string(), "Database error".to_string());
            }
        }
    })();
    Ok(web::Json(json!({
        "status": status,
        "reason": fail_reason,
    })))
}
