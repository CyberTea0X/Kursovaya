use std::fs;

use crate::{
    database::{self, DBconfig, User},
    email, passwords,
};
use actix_web::{post, web, Responder, Result as ActxResult};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug, Default)]
pub struct FindUserRequest {
    id: Option<u32>,
    email: Option<String>,
}

#[post("delete/{email}/{password}")]
pub(crate) async fn delete_user_service(
    path: web::Path<(String, String)>,
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let (email, password) = path.into_inner();
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
        let password_check = passwords::check_password(&mut connection, &email, &password);
        match (password_check.0.as_str(), password_check.1.as_str()) {
            ("OK", "") => {}
            _ => return password_check,
        }
        let id = match database::user_email_to_id(&mut connection, &email) {
            Ok(Some(id)) => id,
            Ok(None) => return ("FAILED".to_owned(), "User already deleted".to_owned()),
            Err(_) => return ("FAILED".to_owned(), "Database error".to_owned()),
        };
        if let Err(err) = database::delete_user(&mut connection, &email) {
            println!("{:?}", err);
            return ("FAILED".to_owned(), "Database error".to_owned());
        }
        let dir_name = format!("users/{}", id);
        fs::remove_dir_all(dir_name).unwrap();
        return ("OK".to_owned(), "".to_owned())
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
        if info.rating.is_some() {
            return (
                "FAILED".to_owned(),
                "You have not enough permissions to modify rating".to_owned(),
            );
        }
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
                "FAILED".to_owned(),
                "Query must not be empty. What you want to edit?".to_owned(),
            );
        }
        if !raw_info
            .iter()
            .flatten()
            .all(|text| database::is_valid_sql(&text))
        {
            return ("FAILED".to_owned(), "Some query attr is invalid".to_owned());
        }
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
        let password_check = passwords::check_password(&mut connection, &email, &password);
        match (password_check.0.as_str(), password_check.1.as_str()) {
            ("OK", "") => {}
            _ => return password_check,
        }
        if let Err(err) = database::edit_user(&mut connection, &email, &info) {
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
        return ("OK".to_owned(), "".to_owned())
    })();
    Ok(web::Json(json!({
        "status": status,
        "reason": fail_reason,
    })))
}

#[post("/profile")] // <- define path parameters
pub(crate) async fn user_profile_service(
    db_config: web::Data<DBconfig>,
    info: web::Query<FindUserRequest>,
) -> ActxResult<impl Responder> {
    let (status, fail_reason, user) = (|| {
        if info.email.is_none() && info.id.is_none() {
            return (
                "FAILED",
                "id and email are not specified. Can't find user without any of them",
                database::User::default(),
            );
        }
        if info.email.is_some() && !email::is_valid_email(info.email.as_ref().unwrap()) {
            return ("FAILED", "Invalid email adresss", database::User::default());
        }
        let connection = database::try_connect(&db_config, 3);
        if connection.is_err() {
            println!("Failed to connect to database");
            return (
                "FAILED",
                "Failed to connect to database",
                database::User::default(),
            );
        }
        let mut connection = connection.unwrap();
        let user = database::find_user(&mut connection, info.email.as_deref(), info.id);
        let user = match user {
            Some(mut user) => {
                hide_attributes(&mut user, &["email", "password"]);
                user
            },
            None => return ("FAILED", "User does not exist", database::User::default())
        };
        return ("OK", "", user);
    })();
    Ok(web::Json(json!({
        "status": status,
        "reason": fail_reason,
        "user": user,
    })))
}

#[post("/visit/{visitor_email}/{visitor_password}/{id}")] // <- define path parameters
pub(crate) async fn visit_user_service(
    path: web::Path<(String, String, u32)>,
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let (email, password, visit_id) = path.into_inner();
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
        let user = database::find_user_by_email(&mut connection, &email);
        if user.is_none() {
            return ("FAILED".to_owned(), "Visitor does not exist".to_owned());
        }
        let user = user.unwrap();
        if user.password != password {
            return ("FAILED".to_owned(), "Invalid password".to_owned());
        }
        let user2 = database::find_user_by_id(&mut connection, visit_id);
        if user2.is_none() {
            return ("FAILED".to_owned(), "Visited does not exist".to_owned());
        }
        let user2 = user2.unwrap();
        if user.id == user2.id {
            return ("FAILED".to_owned(), "Cannot visit yourself".to_owned());
        }
        if database::visit_exists(&mut connection, &user.email, user2.id) {
            return ("FAILED".to_owned(), "Already visited".to_owned());
        }
        let info = database::EditRequest {
            username: None,
            email: None,
            password: None,
            firstname: None,
            lastname: None,
            rating: Some(user.rating + 1),
            about: None,
            age: None,
            gender: None,
            reg_date: None,
        };
        if database::add_visit(&mut connection, &user.email, user2.id).is_err() {
            return ("FAILED".to_owned(), "Database error".to_owned());
        }
        if database::edit_user(&mut connection, &email, &info).is_err() {
            return ("FAILED".to_owned(), "Database error".to_owned());
        }
        return ("OK".to_owned(), "".to_owned());
    })();
    Ok(web::Json(json!({
        "status": status,
        "reason": fail_reason,
    })))
}

pub fn hide_attributes(user: &mut User, hidden_attributes: &[&str]) {
    for attribute in hidden_attributes {
        match *attribute {
            "id" => user.id = 0,
            "username" => user.username = "secret".to_string(),
            "email" => user.email = "secret".to_string(),
            "password" => user.password = "secret".to_string(),
            "firstname" => user.firstname = None,
            "lastname" => user.lastname = None,
            "rating" => user.rating = 0,
            "about" => user.about = None,
            "age" => user.age = None,
            "gender" => user.gender = None,
            "last_online" => user.last_online = None,
            "reg_date" => user.reg_date = None,
            _ => (),
        }
    }
}