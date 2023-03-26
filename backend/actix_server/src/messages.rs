use std::collections::HashMap;

use crate::database::{self, Chat, DBconfig, User, ReadMessagesRequest};
use crate::{email, passwords, user};
use actix_web::{
    dev::ConnectionInfo, get, post, web, App, HttpResponse, HttpServer, Responder,
    Result as ActxResult,
};
use mysql::Conn;
use serde::{Deserialize, Serialize};
use serde_json::json;
use similar_string::compare_similarity;
#[post("/read/{email1}/{password}/{email2}")]
pub async fn read_messages_service(
    path: web::Path<(String, String, String)>,
    db_config: web::Data<DBconfig>,
    messages: web::Json<ReadMessagesRequest>,
) -> ActxResult<impl Responder> {
    let (status, fail_reason) = (|| {
        let (email1, password, email2) = path.into_inner();
        let messages = match &messages.id_list {
            Some(messages) => messages,
            None => return (
                "FAILED".to_owned(),
                "No messages to read specified. Plese specify some through query 'id_list'".to_owned()
            )
        };
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
            None => {
                return (
                    "FAILED".to_owned(),
                    "User1 does not exist".to_owned(),
                )
            }
        };
        let user2 = match database::find_user_by_email(&mut connection, &email2, true) {
            Some(user) => user,
            None => {
                return (
                    "FAILED".to_owned(),
                    "User2 does not exist".to_owned(),
                )
            }
        };
        let chat = match database::find_chat(&mut connection, user1.id, user2.id) {
            Ok(Some(chat)) => chat,
            Ok(None) => {
                return (
                    "FAILED".to_owned(),
                    "Chat does not exist".to_owned(),
                )
            }
            Err(_) => return ("FAILED".to_owned(), "Database error".to_owned()),
        };
        if database::mark_messages_as_read(&mut connection, chat.id, messages).is_err() {
            println!("{:?}", database::mark_messages_as_read(&mut connection, chat.id, messages));
            return ("FAILED".to_owned(), "Database error".to_owned());
        }
        return ("OK".to_owned(), "".to_owned());

    })();
    Ok(web::Json(json!({
        "status": status,
        "reason": fail_reason,
    })))
}

#[post("/get/{email1}/{password}/{email2}")]
pub async fn get_messages_service(
    path: web::Path<(String, String, String)>,
    db_config: web::Data<DBconfig>,
    mut query: web::Query<HashMap<String, usize>>,
) -> ActxResult<impl Responder> {
    let (status, fail_reason, messages) = (|| {
        let (email1, password, email2) = path.into_inner();
        if !email::is_valid_email(email1.as_str()) {
            return (
                "FAILED".to_owned(),
                "Invalid email adress of first user".to_owned(),
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
        if !email::is_valid_email(email2.as_str()) {
            return (
                "FAILED".to_owned(),
                "Invalid email adress of second user".to_owned(),
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
        let user1 = match database::find_user_by_email(&mut connection, &email1, true) {
            Some(user) => user,
            None => {
                return (
                    "FAILED".to_owned(),
                    "User1 does not exist".to_owned(),
                    Vec::new(),
                )
            }
        };
        let user2 = match database::find_user_by_email(&mut connection, &email2, true) {
            Some(user) => user,
            None => {
                return (
                    "FAILED".to_owned(),
                    "User2 does not exist".to_owned(),
                    Vec::new(),
                )
            }
        };
        let chat = match database::find_chat(&mut connection, user1.id, user2.id) {
            Ok(Some(chat)) => chat,
            Ok(None) => {
                return (
                    "FAILED".to_owned(),
                    "Chat does not exist".to_owned(),
                    Vec::new(),
                )
            }
            Err(_) => return ("FAILED".to_owned(), "Database error".to_owned(), Vec::new()),
        };
        let start = query.remove("start");
        let end = query.remove("end");

        let messages = match database::get_messages(&mut connection, chat.id, start, end) {
            Ok(messages) => messages,
            Err(_) => return ("FAILED".to_owned(), "Database error".to_owned(), Vec::new()),
        };
        return ("OK".to_owned(), "".to_owned(), messages);
    })();
    Ok(web::Json(json!({
        "status": status,
        "reason": fail_reason,
        "messages": messages,
    })))
}

#[post("/send/{email1}/{password}/{email2}")]
pub async fn send_message_service(
    path: web::Path<(String, String, String)>,
    db_config: web::Data<DBconfig>,
    mut message: web::Query<HashMap<String, String>>,
) -> ActxResult<impl Responder> {
    let (status, fail_reason) = (|| {
        let (email1, password, email2) = path.into_inner();
        let message = match message.remove("content") {
            Some(value) => value,
            None => {
                return (
                    "FAILED".to_owned(),
                    "No content specified. Please specify content through query 'content'"
                        .to_owned(),
                )
            }
        };
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
        return ("OK".to_owned(), "".to_owned());
    })();
    Ok(web::Json(json!({
        "status": status,
        "reason": fail_reason,
    })))
}
