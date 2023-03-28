use std::fs;

use crate::{
    auth::auth_get_user_connect,
    database::{self, DBconfig, User},
    email,
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
        let (_, mut connection) = match auth_get_user_connect(&email, &password, &db_config, 3) {
            Ok((user, connection)) => (user, connection),
            Err(err) => return ("FAILED".to_owned(), err.to_string()),
        };
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
        return ("OK".to_owned(), "".to_owned());
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
        let (_, mut connection) = match auth_get_user_connect(&email, &password, &db_config, 3) {
            Ok((user, connection)) => (user, connection),
            Err(err) => return ("Failed".to_owned(), err.to_string()),
        };
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
        return ("OK".to_owned(), "".to_owned());
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
                "FAILED".to_owned(),
                "id and email are not specified. Can't find user without any of them".to_owned(),
                database::User::default(),
            );
        }
        if info.email.is_some() && !email::is_valid_email(info.email.as_ref().unwrap()) {
            return (
                "FAILED".to_owned(),
                "Invalid email adresss".to_owned(),
                database::User::default(),
            );
        }
        let mut connection = match database::try_connect(&db_config, 3) {
            Ok(conn) => conn,
            Err(_) => {
                println!("Failed to connect to database");
                return (
                    "FAILED".to_owned(),
                    "Failed to connect to database".to_owned(),
                    database::User::default(),
                );
            }
        };
        let user = database::find_user(&mut connection, info.email.as_deref(), info.id);
        let user = match user {
            Some(mut user) => {
                hide_attributes(&mut user, &["email", "password"]);
                user
            }
            None => {
                return (
                    "FAILED".to_owned(),
                    "User does not exist".to_owned(),
                    database::User::default(),
                )
            }
        };
        return ("OK".to_owned(), "".to_owned(), user);
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
        let (user, mut connection) = match auth_get_user_connect(&email, &password, &db_config, 3) {
            Ok((user, connection)) => (user, connection),
            Err(err) => return ("Failed".to_owned(), err.to_string()),
        };
        let user2 = match database::find_user_by_id(&mut connection, visit_id) {
            Some(user) => user,
            None => return ("FAILED".to_owned(), "Visited does not exist".to_owned()),
        };
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
