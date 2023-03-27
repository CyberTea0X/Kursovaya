use crate::database::{self, DBconfig, User};
use crate::user::hide_attributes;
use actix_web::{post, web, Responder, Result as ActxResult};
use mysql::Conn;
use serde::{Deserialize, Serialize};
use serde_json::json;
use similar_string::compare_similarity;

#[derive(Debug, Serialize, Deserialize)]
pub struct SeachName {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
}

#[post("/popular")]
pub async fn search_popular_service(db_config: web::Data<DBconfig>) -> ActxResult<impl Responder> {
    let (status, fail_reason, items) = (|| {
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
        let items = search_popular(&mut connection);
        if items.is_err() {
            return ("FAILED".to_owned(), "Database error".to_owned(), Vec::new());
        }
        return ("OK".to_owned(), "".to_owned(), items.unwrap());
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
        let users = search_login(&mut connection, &login);
        if users.is_err() {
            return ("FAILED".to_owned(), "Database error".to_owned(), Vec::new());
        }
        return ("OK".to_owned(), "".to_owned(), users.unwrap());
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
        let users = search_name(&mut connection, &info);
        if users.is_err() {
            return ("FAILED".to_owned(), "Database error".to_owned(), Vec::new());
        }
        return ("OK".to_owned(), "".to_owned(), users.unwrap());
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
        let users = search_text(&mut connection, &text);
        if users.is_err() {
            return ("FAILED".to_owned(), "Database error".to_owned(), Vec::new());
        }
        return ("OK".to_owned(), "".to_owned(), users.unwrap());
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
