use crate::database::{self, DBconfig};
use crate::{email, passwords};
use actix_web::{
    post, web, Responder,
    Result as ActxResult,
};
use serde_json::json;

#[post("/user/{email1}/{password}")]
pub async fn get_user_chats_service(
    path: web::Path<(String, String)>,
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let (status, fail_reason, chats) = (|| {
        let (email, password) = path.into_inner();
        if !email::is_valid_email(email.as_str()) {
            return (
                "FAILED".to_owned(),
                "Invalid email adress".to_owned(),
                Vec::new(),
            );
        }
        if !passwords::is_valid_password(&password) {
            return (
                "FAILED".to_owned(),
                "Password contains invalid characters or too small".to_owned(),
                Vec::new(),
            );
        }
        let connection = database::try_connect(&db_config, 3);
        if connection.is_err() {
            println!("Failed to connect to database");
            return (
                "FAILED".to_owned(),
                "Failed to connect to database".to_owned(),
                Vec::new(),
            );
        }
        let mut connection = connection.unwrap();
        let user = database::find_user_by_email(&mut connection, &email, false);
        if user.is_none() {
            return (
                "FAILED".to_owned(),
                "User does not exist".to_owned(),
                Vec::new(),
            );
        }
        let user = user.unwrap();
        if user.password != password {
            return (
                "FAILED".to_owned(),
                "Invalid password".to_owned(),
                Vec::new(),
            );
        }
        let chats = database::get_user_chats(&mut connection, user.id);
        if chats.is_err() {
            return ("FAILED".to_owned(), "Database error".to_owned(), Vec::new());
        }
        return ("OK".to_owned(), "".to_owned(), chats.unwrap());
    })();
    Ok(web::Json(json!({
        "status": status,
        "reason": fail_reason,
        "chats": chats
    })))
}

#[post("/delete/{email1}/{password}/{email2}")]
pub async fn delete_chat_service(
    path: web::Path<(String, String, String)>,
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let (status, fail_reason) = (|| {
        let (email1, password, email2) = path.into_inner();
        if !email::is_valid_email(email1.as_str()) {
            return (
                "FAILED".to_owned(),
                "Invalid email adress of first user".to_owned(),
            );
        }
        if !email::is_valid_email(email2.as_str()) {
            return (
                "FAILED".to_owned(),
                "Invalid email adress of second user".to_owned(),
            );
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
        let user1 = database::find_user_by_email(&mut connection, &email1, false);
        if user1.is_none() {
            return ("FAILED".to_owned(), "User1 does not exist".to_owned());
        }
        let user1 = user1.unwrap();
        if user1.password != password {
            return ("FAILED".to_owned(), "Invalid password".to_owned());
        }
        let user2 = database::find_user_by_email(&mut connection, &email2, true);
        if user2.is_none() {
            return ("FAILED".to_owned(), "User2 does not exist".to_owned());
        }
        let user2 = user2.unwrap();
        if !database::is_chat_exists(&mut connection, user1.id, user2.id) {
            return ("FAILED".to_owned(), "Chat does not exist".to_owned());
        }
        if !database::delete_chat(&mut connection, user1.id, user2.id).is_ok() {
            return ("FAILED".to_owned(), "Database error".to_owned());
        }
        return ("OK".to_owned(), "".to_owned());
    })();
    Ok(web::Json(json!({
        "status": status,
        "reason": fail_reason,
    })))
}

#[post("/create/{email1}/{password}/{email2}")]
pub async fn create_chat_service(
    path: web::Path<(String, String, String)>,
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let (status, fail_reason) = (|| {
        let (email1, password, email2) = path.into_inner();
        if !email::is_valid_email(email1.as_str()) {
            return (
                "FAILED".to_owned(),
                "Invalid email adress of first user".to_owned(),
            );
        }
        if !email::is_valid_email(email2.as_str()) {
            return (
                "FAILED".to_owned(),
                "Invalid email adress of second user".to_owned(),
            );
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
        let user1 = database::find_user_by_email(&mut connection, &email1, false);
        if user1.is_none() {
            return ("FAILED".to_owned(), "User1 does not exist".to_owned());
        }
        let user1 = user1.unwrap();
        if user1.password != password {
            return ("FAILED".to_owned(), "Invalid password".to_owned());
        }
        let user2 = database::find_user_by_email(&mut connection, &email2, true);
        if user2.is_none() {
            return ("FAILED".to_owned(), "User2 does not exist".to_owned());
        }
        let user2 = user2.unwrap();
        if database::is_chat_exists(&mut connection, user1.id, user2.id) {
            return ("FAILED".to_owned(), "Chat already exists".to_owned());
        }
        if !database::create_chat(&mut connection, user1.id, user2.id).is_ok() {
            return ("FAILED".to_owned(), "Database error".to_owned());
        }
        return ("OK".to_owned(), "".to_owned());
    })();
    Ok(web::Json(json!({
        "status": status,
        "reason": fail_reason,
    })))
}

#[post("/exists/{email1}/{password}/{email2}")]
pub async fn is_chat_exists_service(
    path: web::Path<(String, String, String)>,
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let (status, fail_reason) = (|| {
        let (email1, password, email2) = path.into_inner();
        if !email::is_valid_email(email1.as_str()) {
            return (
                "FAILED".to_owned(),
                "Invalid email adress of first user".to_owned(),
            );
        }
        if !email::is_valid_email(email2.as_str()) {
            return (
                "FAILED".to_owned(),
                "Invalid email adress of second user".to_owned(),
            );
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
        let user1 = database::find_user_by_email(&mut connection, &email1, false);
        if user1.is_none() {
            return ("FAILED".to_owned(), "User1 does not exist".to_owned());
        }
        let user1 = user1.unwrap();
        if user1.password != password {
            return ("FAILED".to_owned(), "Invalid password".to_owned());
        }
        let user2 = database::find_user_by_email(&mut connection, &email2, true);
        if user2.is_none() {
            return ("FAILED".to_owned(), "User2 does not exist".to_owned());
        }
        let user2 = user2.unwrap();
        if database::is_chat_exists(&mut connection, user1.id, user2.id) {
            return ("OK".to_owned(), "Exists".to_owned());
        }
        return ("OK".to_owned(), "Does not exist".to_owned());
    })();
    Ok(web::Json(json!({
        "status": status,
        "reason": fail_reason,
    })))
}
