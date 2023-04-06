use std::collections::HashMap;

use crate::database::{self, DBconfig, User};
use crate::user::hide_attributes;
use actix_web::{post, web, Responder, Result as ActxResult};
use mysql::Conn;
use serde::{Deserialize, Serialize};
use serde_json::json;
use similar_string::{compare_similarity, get_similarity_ratings};

#[derive(Debug, Serialize, Deserialize)]
pub struct SeachName {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
}

#[post("/tags/{tags}")]
pub async fn search_tags_service(
    db_config: web::Data<DBconfig>,
    path: web::Path<String>,
) -> ActxResult<impl Responder> {
    let (status, fail_reason, items) = (|| {
        let tags = path.into_inner();
        let mut connection = match database::try_connect(&db_config, 3) {
            Ok(conn) => conn,
            Err(_) => {
                println!("Failed to connect to database");
                return (
                    "FAILED".to_owned(),
                    "Failed to connect to database".to_owned(),
                    Vec::new(),
                );
            }
        };
        let users = match search_tags(&mut connection, &tags) {
            Ok(users) => users,
            Err(_) => return ("FAILED".to_owned(), "Database error".to_owned(), Vec::new()),
        };
        return ("OK".to_owned(), "".to_owned(), users);
    })();
    Ok(web::Json(json!({
        "items": items,
        "status": status,
        "reason": fail_reason,
    })))
}

#[post("/popular")]
pub async fn search_popular_service(db_config: web::Data<DBconfig>) -> ActxResult<impl Responder> {
    let (status, fail_reason, items) = (|| {
        let mut connection = match database::try_connect(&db_config, 3) {
            Ok(conn) => conn,
            Err(_) => {
                println!("Failed to connect to database");
                return (
                    "FAILED".to_owned(),
                    "Failed to connect to database".to_owned(),
                    Vec::new(),
                );
            }
        };
        let items = match search_popular(&mut connection) {
            Ok(items) => items,
            Err(_) => return ("FAILED".to_owned(), "Database error".to_owned(), Vec::new()),
        };
        return ("OK".to_owned(), "".to_owned(), items);
    })();
    Ok(web::Json(json!({
        "items": items,
        "status": status,
        "reason": fail_reason,
    })))
}

#[post("/login/{login}")]
pub async fn search_login_service(
    db_config: web::Data<DBconfig>,
    path: web::Path<String>,
) -> ActxResult<impl Responder> {
    let (status, fail_reason, items) = (|| {
        let login = path.into_inner();
        let mut connection = match database::try_connect(&db_config, 3) {
            Ok(conn) => conn,
            Err(_) => {
                println!("Failed to connect to database");
                return (
                    "FAILED".to_owned(),
                    "Failed to connect to database".to_owned(),
                    Vec::new(),
                );
            }
        };
        let users = match search_login(&mut connection, &login) {
            Ok(users) => users,
            Err(_) => return ("FAILED".to_owned(), "Database error".to_owned(), Vec::new()),
        };
        return ("OK".to_owned(), "".to_owned(), users);
    })();
    Ok(web::Json(json!({
        "items": items,
        "status": status,
        "reason": fail_reason,
    })))
}

#[post("/name")]
pub async fn search_name_service(
    db_config: web::Data<DBconfig>,
    info: web::Query<SeachName>,
) -> ActxResult<impl Responder> {
    let (status, fail_reason, items) = (|| {
        let mut connection = match database::try_connect(&db_config, 3) {
            Ok(conn) => conn,
            Err(_) => {
                println!("Failed to connect to database");
                return (
                    "FAILED".to_owned(),
                    "Failed to connect to database".to_owned(),
                    Vec::new(),
                );
            }
        };
        let users = match search_name(&mut connection, &info) {
            Ok(users) => users,
            Err(_) => return ("FAILED".to_owned(), "Database error".to_owned(), Vec::new()),
        };
        return ("OK".to_owned(), "".to_owned(), users);
    })();
    Ok(web::Json(json!({
        "items": items,
        "status": status,
        "reason": fail_reason,
    })))
}

#[post("/text/{text}")]
pub async fn search_text_service(
    db_config: web::Data<DBconfig>,
    path: web::Path<String>,
) -> ActxResult<impl Responder> {
    let (status, fail_reason, items) = (|| {
        let text = path.into_inner();
        let mut connection = match database::try_connect(&db_config, 3) {
            Ok(conn) => conn,
            Err(_) => {
                println!("Failed to connect to database");
                return (
                    "FAILED".to_owned(),
                    "Failed to connect to database".to_owned(),
                    Vec::new(),
                );
            }
        };
        let users = match search_text(&mut connection, &text) {
            Ok(users) => users,
            Err(_) => return ("FAILED".to_owned(), "Database error".to_owned(), Vec::new()),
        };
        return ("OK".to_owned(), "".to_owned(), users);
    })();
    Ok(web::Json(json!({
        "items": items,
        "status": status,
        "reason": fail_reason,
    })))
}

pub fn search_text(connection: &mut Conn, text: &str) -> Result<Vec<User>, mysql::Error> {
    let mut query_result = database::get_all_users(connection)?;
    query_result
        .iter_mut()
        .for_each(|user| hide_attributes(user, &["email", "password"]));
    query_result.sort_by_key(|user| {
        let user_params = [
            user.username.clone(),
            user.firstname.as_deref().unwrap_or_default().to_owned(),
            user.lastname.as_deref().unwrap_or_default().to_owned(),
            user.gender.as_deref().unwrap_or_default().to_owned(),
            user.about.as_deref().unwrap_or_default().to_owned(),
            user.age.as_deref().unwrap_or_default().to_owned(),
        ]
        .join(" ");
        let similarity = -compare_similarity(&text.to_lowercase(), &user_params.to_lowercase());
        (similarity * 10.0).round() as i32
    });
    Ok(query_result)
}

pub fn search_tags(connection: &mut Conn, tags: &str) -> Result<Vec<User>, mysql::Error> {
    let mut users = database::get_all_users(connection)?;
    users
        .iter_mut()
        .for_each(|user| hide_attributes(user, &["email", "password"]));
    let tags: Vec<&str> = tags.split(",").collect();
    let mut similarity_hash = HashMap::new();
    for tag_info in database::get_user_tags_table(connection)? {
        let user_id = tag_info.user_id;
        let user_tags: Vec<&str> = tag_info.tags.split(",").collect();
        let mut similarity = 0.0;
        for tag in &tags {
            similarity += get_similarity_ratings(tag, &user_tags).unwrap_or_default().iter().sum::<f64>();
        }
        let similarity = ((similarity * 10.0) as f64).round() as i32;
        similarity_hash.insert(user_id,  similarity);
    }
    users.sort_by_key(|user| {
        -similarity_hash.get(&(user.id as u64)).unwrap_or(&0)
    });
    Ok(users)
}

pub fn search_name(connection: &mut Conn, info: &SeachName) -> Result<Vec<User>, mysql::Error> {
    let mut query_result = database::get_all_users(connection)?;
    query_result
        .iter_mut()
        .for_each(|user| hide_attributes(user, &["email", "password"]));
    let firstname = info.firstname.as_deref().unwrap_or_default().to_owned();
    let lastname = info.lastname.as_deref().unwrap_or_default().to_owned();
    let req_first_and_lastname = format!("{firstname} {lastname}").to_lowercase();
    query_result.sort_by_key(|user| {
        let firstname = user.firstname.as_deref().unwrap_or_default().to_owned();
        let lastname = user.lastname.as_deref().unwrap_or_default().to_owned();
        let first_and_lastname = format!("{firstname} {lastname}").to_lowercase();
        let similarity = -compare_similarity(&req_first_and_lastname, &first_and_lastname);
        (similarity * 10.0).round() as i32
    });
    Ok(query_result)
}

pub fn search_login(connection: &mut Conn, login: &str) -> Result<Vec<User>, mysql::Error> {
    let mut query_result = database::get_all_users(connection)?;
    query_result
        .iter_mut()
        .for_each(|user| hide_attributes(user, &["email", "password"]));
    query_result.sort_by_key(|user| {
        -f64::round(compare_similarity(&user.username.to_lowercase(), &login.to_lowercase()) * 10.0)
            as i32
    });
    Ok(query_result)
}

pub fn search_popular(connection: &mut Conn) -> Result<Vec<User>, mysql::Error> {
    let mut query_result = database::get_all_users(connection)?;
    query_result
        .iter_mut()
        .for_each(|user| hide_attributes(user, &["email", "password"]));
    query_result.sort_by_key(|user| -(user.rating as i32));
    Ok(query_result)
}
