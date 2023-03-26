use std::collections::HashMap;

use crate::database::{self, Chat, DBconfig, User};
use crate::{email, passwords, user};
use actix_web::{
    dev::ConnectionInfo, get, post, web, App, HttpResponse, HttpServer, Responder,
    Result as ActxResult,
};
use mysql::Conn;
use serde::{Deserialize, Serialize};
use serde_json::json;
use similar_string::compare_similarity;

#[post("/get/{email1}/{password}/{email2}")]
pub async fn get_messages_service(
    path: web::Path<(String, String, String)>,
    db_config: web::Data<DBconfig>,
    mut query: web::Query<HashMap<String, usize>>
) -> ActxResult<impl Responder> {
    let (status, fail_reason, messages) = (|| {
        let (email1, password, email2) = path.into_inner();
        if !email::is_valid_email(email1.as_str()) {
            return (
                "FAILED".to_owned(),
                "Invalid email adress of first user".to_owned(),
                Vec::new()
            );
        }
        if !passwords::is_valid_password(&password) {
            return (
                "FAILED".to_owned(),
                "Password contains invalid characters or too small".to_owned(),
                Vec::new()
            );
        }
        if !email::is_valid_email(email2.as_str()) {
            return (
                "FAILED".to_owned(),
                "Invalid email adress of second user".to_owned(),
                Vec::new()
            );
        }
        let connection = database::try_connect(&db_config, 3);
        if connection.is_err() {
            println!("Failed to connect to database");
            return (
                "FAILED".to_owned(),
                "Failed to connect to database".to_owned(),
                Vec::new()
            );
        }
        let mut connection = connection.unwrap();
        let user1 = match database::find_user_by_email(&mut connection, &email1, true) {
            Some(user) => user,
            None => return ("FAILED".to_owned(), "User1 does not exist".to_owned(), Vec::new()),
        };
        let user2 = match database::find_user_by_email(&mut connection, &email2, true) {
            Some(user) => user,
            None => return ("FAILED".to_owned(), "User2 does not exist".to_owned(), Vec::new()),
        };
        let chat = match database::find_chat(&mut connection, user1.id, user2.id) {
            Ok(Some(chat)) => chat,
            Ok(None) => return ("FAILED".to_owned(), "Chat does not exist".to_owned(), Vec::new()),
            Err(_) => return ("FAILED".to_owned(), "Database error".to_owned(), Vec::new()),
        };
        let start = query.remove("start");
        let end = query.remove("end");

        let messages = match database::get_messages(&mut connection, chat.id, start, end) {
            Ok(messages) => messages,
            Err(_) => return ("FAILED".to_owned(), "Database error".to_owned(), Vec::new()),
        };
        return ("Ok".to_owned(), "".to_owned(), messages);
    })();
    Ok(web::Json(json!({
        "status": status,
        "reason": fail_reason,
        "messages": messages,
    })))
}

#[post("/send/{email1}/{password}/{email2}/{message}")]
pub async fn send_message_service(
    path: web::Path<(String, String, String, String)>,
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let (status, fail_reason) = (|| {
        let (email1, password, email2, message) = path.into_inner();
        if !email::is_valid_email(email1.as_str()) {
            return (
                "FAILED".to_owned(),
                "Invalid email adress of first user".to_owned(),
            );
        }
        if !passwords::is_valid_password(&password) {
            return (
                "FAILED".to_owned(),
                "Password contains invalid characters or too small".to_owned(),
            );
        }
        if !email::is_valid_email(email2.as_str()) {
            return (
                "FAILED".to_owned(),
                "Invalid email adress of second user".to_owned(),
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
        let user1 = match database::find_user_by_email(&mut connection, &email1, true) {
            Some(user) => user,
            None => return ("FAILED".to_owned(), "User1 does not exist".to_owned()),
        };
        let user2 = match database::find_user_by_email(&mut connection, &email2, true) {
            Some(user) => user,
            None => return ("FAILED".to_owned(), "User2 does not exist".to_owned()),
        };
        let chat = match database::find_chat(&mut connection, user1.id, user2.id) {
            Ok(Some(chat)) => chat,
            Ok(None) => return ("FAILED".to_owned(), "Chat does not exist".to_owned()),
            Err(_) => return ("FAILED".to_owned(), "Database error".to_owned()),
        };
        let result = database::send_message(
            &mut connection,
            chat.id,
            user1.id,
            &user1.username,
            &message,
        );
        if result.is_err() {
            println!("{:?}", result);
            return ("FAILED".to_owned(), "Database error".to_owned());
        }
        return ("Ok".to_owned(), "".to_owned());
    })();
    Ok(web::Json(json!({
        "status": status,
        "reason": fail_reason,
    })))
}
