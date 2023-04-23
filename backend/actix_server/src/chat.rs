use crate::{
    auth::auth_get_user_connect,
    database::{self, DBconfig},
};
use actix_web::{post, web, Responder, Result as ActxResult};
use serde_json::json;


#[post("/user/{email1}/{password}")]
pub async fn get_user_chats_service(
    path: web::Path<(String, String)>,
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let (status, fail_reason, chats) = (|| {
        let (email, password) = path.into_inner();
        let (user, mut connection) = match auth_get_user_connect(&email, &password, &db_config, 3) {
            Ok((user, connection)) => (user, connection),
            Err(err) => return ("FAILED".to_owned(), err.to_string(), Vec::new()),
        };
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

#[post("/delete/{email1}/{password}/{id}")]
pub async fn delete_chat_service(
    path: web::Path<(String, String, u32)>,
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let (status, fail_reason) = (|| {
        let (email, password, id) = path.into_inner();
        let (user1, mut connection) = match auth_get_user_connect(&email, &password, &db_config, 3)
        {
            Ok((user, connection)) => (user, connection),
            Err(err) => return ("FAILED".to_owned(), err.to_string()),
        };
        let user2 = match database::find_user_by_id(&mut connection, id) {
            Some(user) => user,
            None => return ("FAILED".to_owned(), "User2 does not exist".to_owned()),
        };
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

#[post("/create/{email1}/{password}/{id}")]
pub async fn create_chat_service(
    path: web::Path<(String, String, u32)>,
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let (status, fail_reason) = (|| {
        let (email, password, id) = path.into_inner();
        let (user1, mut connection) = match auth_get_user_connect(&email, &password, &db_config, 3)
        {
            Ok((user, connection)) => (user, connection),
            Err(err) => return ("FAILED".to_owned(), err.to_string()),
        };
        let user2 = match database::find_user_by_id(&mut connection, id) {
            Some(user) => user,
            None => return ("FAILED".to_owned(), "User2 does not exist".to_owned()),
        };
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

#[post("/exists/{email1}/{password}/{id}")]
pub async fn is_chat_exists_service(
    path: web::Path<(String, String, u32)>,
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let (status, fail_reason) = (|| {
        let (email, password, id) = path.into_inner();
        let (user1, mut connection) = match auth_get_user_connect(&email, &password, &db_config, 3)
        {
            Ok((user, connection)) => (user, connection),
            Err(err) => return ("FAILED".to_owned(), err.to_string()),
        };
        let user2 = match database::find_user_by_id(&mut connection, id) {
            Some(user) => user,
            None => return ("FAILED".to_owned(), "User2 does not exist".to_owned()),
        };
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
