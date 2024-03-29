use std::collections::HashMap;

use crate::auth::auth_get_user_connect;
use crate::database::{self, DBconfig, Message};
use actix_web::{post, web, Responder, Result as ActxResult};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ReadMessagesRequest {
    pub id_list: Option<Vec<u32>>,
}



#[post("/unread/{email1}/{password}/{user_id}")]
pub async fn count_unread_service(
    path: web::Path<(String, String, u32)>,
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let (email, password, user_id) = path.into_inner();
    let (status, fail_reason, count) = (|| {
        let (user1, mut connection) = match auth_get_user_connect(&email, &password, &db_config, 3)
        {
            Ok((user, connection)) => (user, connection),
            Err(err) => return ("FAILED".to_owned(), err.to_string(), 0),
        };
        let user2 = match database::find_user_by_id(&mut connection, user_id) {
            Some(user) => user,
            None => return ("FAILED".to_owned(), "User2 does not exist".to_owned(), 0),
        };
        let chat = match database::find_chat(&mut connection, user1.id, user2.id) {
            Ok(Some(chat)) => chat,
            Ok(None) => return ("FAILED".to_owned(), "Chat does not exist".to_owned(), 0),
            Err(_) => return ("FAILED".to_owned(), "Database error".to_owned(), 0),
        };
        match database::count_unread_messages(&mut connection, chat.id) {
            Ok(count) => return ("OK".to_owned(), "".to_owned(), count),
            Err(_) => return ("FAILED".to_owned(), "Database error".to_owned(), 0)
        }
    })();
    Ok(web::Json(json!({
        "status": status,
        "reason": fail_reason,
        "count": count
    })))
}

#[post("/last/{email1}/{password}/{user_id}")]
pub async fn last_chat_message_service(
    path: web::Path<(String, String, u32)>,
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let (status, fail_reason, message) = (|| {
        let (email, password, user_id) = path.into_inner();
        let (user1, mut connection) = match auth_get_user_connect(&email, &password, &db_config, 3)
        {
            Ok((user, connection)) => (user, connection),
            Err(err) => return ("FAILED".to_owned(), err.to_string(), None),
        };
        let user2 = match database::find_user_by_id(&mut connection, user_id) {
            Some(user) => user,
            None => {
                return (
                    "FAILED".to_owned(),
                    "User2 does not exist".to_owned(),
                    None,
                )
            }
        };
        let chat = match database::find_chat(&mut connection, user1.id, user2.id) {
            Ok(Some(chat)) => chat,
            Ok(None) => {
                return (
                    "FAILED".to_owned(),
                    "Chat does not exist".to_owned(),
                    None,
                )
            }
            Err(_) => return ("FAILED".to_owned(), "Database error".to_owned(), None),
        };
        
        match database::get_last_chat_message(&mut connection, chat.id) {
            Ok(Some(message)) => return ("OK".to_owned(), "".to_owned(), Some(message)),
            Ok(None) => return ("FAILED".to_owned(), "Chat is empty".to_owned(), None),
            Err(_) => return ("FAULED".to_owned(), "Database error".to_owned(), None)
        }
        
    })();
    if message.is_none() {
        return Ok(web::Json(json!({
            "status": status,
            "reason": fail_reason,
            "message": "",
        })))
    }
    Ok(web::Json(json!({
        "status": status,
        "reason": fail_reason,
        "message": message.unwrap(),
    })))
}

#[post("/readall/{email1}/{password}/{user_id}")]
pub async fn read_all_messages_service(
    path: web::Path<(String, String, u32)>,
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let (status, fail_reason) = (|| {
        let (email, password, user_id) = path.into_inner();
        let (user1, mut connection) = match auth_get_user_connect(&email, &password, &db_config, 3)
        {
            Ok((user, connection)) => (user, connection),
            Err(err) => return ("FAILED".to_owned(), err.to_string()),
        };
        let user2 = match database::find_user_by_id(&mut connection, user_id) {
            Some(user) => user,
            None => return ("FAILED".to_owned(), "User2 does not exist".to_owned()),
        };
        let chat = match database::find_chat(&mut connection, user1.id, user2.id) {
            Ok(Some(chat)) => chat,
            Ok(None) => return ("FAILED".to_owned(), "Chat does not exist".to_owned()),
            Err(_) => return ("FAILED".to_owned(), "Database error".to_owned()),
        };
        let result = database::read_all_messages(&mut connection, chat.id, user2.id);
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

#[post("/read/{email1}/{password}/{user_id}")]
pub async fn read_messages_service(
    path: web::Path<(String, String, u32)>,
    db_config: web::Data<DBconfig>,
    messages: web::Json<ReadMessagesRequest>,
) -> ActxResult<impl Responder> {
    let (status, fail_reason) = (|| {
        let (email, password, user_id) = path.into_inner();
        let messages = match &messages.id_list {
            Some(messages) => messages,
            None => {
                return (
                    "FAILED".to_owned(),
                    "No messages to read specified. Plese specify some through query 'id_list'"
                        .to_owned(),
                )
            }
        };
        let (user1, mut connection) = match auth_get_user_connect(&email, &password, &db_config, 3)
        {
            Ok((user, connection)) => (user, connection),
            Err(err) => return ("FAILED".to_owned(), err.to_string()),
        };
        let user2 = match database::find_user_by_id(&mut connection, user_id) {
            Some(user) => user,
            None => return ("FAILED".to_owned(), "User2 does not exist".to_owned()),
        };
        let chat = match database::find_chat(&mut connection, user1.id, user2.id) {
            Ok(Some(chat)) => chat,
            Ok(None) => return ("FAILED".to_owned(), "Chat does not exist".to_owned()),
            Err(_) => return ("FAILED".to_owned(), "Database error".to_owned()),
        };
        let result = database::mark_messages_as_read(&mut connection, chat.id, messages);
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

#[post("/get/{email1}/{password}/{user_id}")]
pub async fn get_messages_service(
    path: web::Path<(String, String, u32)>,
    db_config: web::Data<DBconfig>,
    mut query: web::Query<HashMap<String, usize>>,
) -> ActxResult<impl Responder> {
    let (status, fail_reason, messages) = (|| {
        let (email, password, user_id) = path.into_inner();
        let (user1, mut connection) = match auth_get_user_connect(&email, &password, &db_config, 3)
        {
            Ok((user, connection)) => (user, connection),
            Err(err) => return ("FAILED".to_owned(), err.to_string(), Vec::new()),
        };
        let user2 = match database::find_user_by_id(&mut connection, user_id) {
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

#[post("/send/{email1}/{password}/{user_id}")]
pub async fn send_message_service(
    path: web::Path<(String, String, u32)>,
    db_config: web::Data<DBconfig>,
    mut message: web::Query<HashMap<String, String>>,
) -> ActxResult<impl Responder> {
    let (status, fail_reason) = (|| {
        let (email, password, user_id) = path.into_inner();
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
        let (user1, mut connection) = match auth_get_user_connect(&email, &password, &db_config, 3)
        {
            Ok((user, connection)) => (user, connection),
            Err(err) => return ("FAILED".to_owned(), err.to_string()),
        };
        let user2 = match database::find_user_by_id(&mut connection, user_id) {
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
