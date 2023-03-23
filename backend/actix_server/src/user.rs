use crate::{
    database::{self, DBconfig},
    email, passwords,
};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result as ActxResult};
use serde::Deserialize;
use serde_json::json;

#[post("delete/{email}/{password}")]
pub(crate) async fn delete_user_service(
    path: web::Path<(String, String)>,
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let (email, password) = path.into_inner();
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
        let password_check = passwords::check_password(&mut connection, &email, &password);
        match (password_check.0.as_str(), password_check.1.as_str()) {
            ("OK", "") => {}
            _ => return password_check,
        }
        match database::delete_user(&mut connection, &email) {
            Ok(_) => return ("OK".to_string(), "".to_string()),
            Err(err) => {
                println!("{:?}", err);
                return ("FAILED".to_string(), "Database error".to_string());
            }
        }
    })();
    Ok(web::Json(json!({
        "status": status,
        "reason": fail_reason,
    })))
}

#[post("/edit/{email}/{password}")]
pub(crate) async fn edit_user_service(
    path: web::Path<(String, String)>,
    info: web::Query<database::EditRequest>,
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let (email, password) = path.into_inner();
    let (status, fail_reason) = (|| {
        let raw_info = [
            info.username.as_ref(),
            info.email.as_ref(),
            info.password.as_ref(),
            info.firstname.as_ref(),
            info.lastname.as_ref(),
            info.about.as_ref(),
            info.age.as_ref(),
            info.gender.as_ref(),
        ];
        if raw_info.iter().flatten().collect::<Vec<&&String>>().len() == 0 {
            return (
                "FAILED".to_string(),
                "Query must not be empty. What you want to edit?".to_string(),
            );
        }
        if !raw_info
            .iter()
            .flatten()
            .all(|text| database::is_valid_sql(&text))
        {
            return (
                "FAILED".to_string(),
                "Some query attr is invalid".to_string(),
            );
        }
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
        let password_check = passwords::check_password(&mut connection, &email, &password);
        match (password_check.0.as_str(), password_check.1.as_str()) {
            ("OK", "") => {}
            _ => return password_check,
        }
        match database::edit_user(&mut connection, &email, &info) {
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

#[post("/profile/{email}")] // <- define path parameters
pub(crate) async fn fetch_user_profile(
    path: web::Path<String>,
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let email = path.into_inner();
    let (status, fail_reason, user) = (|| {
        let connection = database::try_connect(&db_config, 3);
        if !email::is_valid_email(email.as_str()) {
            return ("FAILED", "Invalid email adresss", database::User::default());
        }
        if connection.is_err() {
            println!("Failed to connect to database");
            return (
                "FAILED",
                "Failed to connect to database",
                database::User::default(),
            );
        }
        let mut connection = connection.unwrap();
        let user = database::find_user(&mut connection, &email);
        if user.is_none() {
            return ("FAILED", "User does not exist", database::User::default());
        }
        let mut user = user.unwrap();
        user.password = "secret!".to_string();
        return ("OK", "", user);
    })();
    Ok(web::Json(json!({
        "status": status,
        "reason": fail_reason,
        "user": user,
    })))
}
