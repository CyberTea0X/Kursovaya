use std::{
    fs::{self, File},
    io::{Read, Write},
};

use crate::{
    auth::auth_get_user_connect,
    database::{self, DBconfig},
    files::save_file,
};
use actix_multipart::Multipart;
use actix_web::{
    get, post, web, App, Error, HttpResponse, HttpServer, Responder, Result as ActxResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageAddRequest {
    pub about: String,
    pub image_name: String,
    pub tags: String,
}

#[post("/data/{image_id}")] // <- define path parameters
async fn image_data_service(
    path: web::Path<(u32)>,
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let image_id = path.into_inner();
    let (status, fail_reason, image) = (|| {
        let mut connection = match database::try_connect(&db_config, 3) {
            Ok(conn) => conn,
            Err(_) => {
                return (
                    "FAILED".to_owned(),
                    "Failed to connect to database".to_owned(),
                    database::ImageData::default(),
                )
            }
        };
        let image_data = match database::get_image(&mut connection, image_id) {
            Ok(Some(image_data)) => image_data,
            Ok(None) => {
                return (
                    "FAILED".to_owned(),
                    "Image not found".to_owned(),
                    database::ImageData::default(),
                )
            }
            Err(_) => {
                return (
                    "FAILED".to_owned(),
                    "Database error".to_owned(),
                    database::ImageData::default(),
                )
            }
        };
        ("OK".to_owned(), "".to_owned(), image_data)
    })();
    Ok(web::Json(json!({
        "status": status,
        "reason": fail_reason,
        "image": image
    })))
}

#[post("/load/{email}/{password}")] // <- define path parameters
async fn load_image_service(
    payload: Multipart,
    path: web::Path<(String, String)>,
    db_config: web::Data<DBconfig>,
    query: web::Query<ImageAddRequest>,
) -> ActxResult<impl Responder> {
    let (email, password) = path.into_inner();
    let mut file_name = String::new();
    let mut user_gallery = String::new();
    let (status, fail_reason) = (|| {
        let (user, mut connection) = match auth_get_user_connect(&email, &password, &db_config, 3) {
            Ok((user, connection)) => (user, connection),
            Err(err) => return ("FAILED".to_owned(), err.to_string()),
        };
        let (about, image_name, tags) = (&query.about, &query.image_name, &query.tags);
        if database::add_image(&mut connection, user.id, &about, &image_name, &tags).is_err() {
            return ("FAILED".to_owned(), "Database error".to_owned());
        }
        file_name = connection.last_insert_id().to_string();
        user_gallery = format!("users/{}/gallery", user.id);
        ("OK".to_owned(), "".to_owned())
    })();
    if !user_gallery.is_empty() {
        if save_file(
            payload,
            &user_gallery,
            &file_name,
            &["png", "jpeg", "jpe", "jpg"],
        )
        .await
        .is_none()
        {
            return Ok(web::Json(json!({
                "status": "FAILED",
                "reason": "Failed to save file",
            })));
        }
    }
    Ok(web::Json(json!({
        "status": status,
        "reason": fail_reason,
    })))
}

#[post("/delete/{email}/{password}/{image_id}")] // <- define path parameters
async fn delete_image_service(
    path: web::Path<(String, String, u32)>,
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let (email, password, img_id) = path.into_inner();
    let (status, fail_reason) = (|| {
        let (user, mut connection) = match auth_get_user_connect(&email, &password, &db_config, 3) {
            Ok((user, connection)) => (user, connection),
            Err(err) => return ("FAILED".to_owned(), err.to_string()),
        };
        let img = match database::get_image(&mut connection, img_id) {
            Ok(Some(img)) => img,
            Ok(None) => return ("FAILED".to_owned(), "Image already deleted".to_owned()),
            Err(_) => return ("FAILED".to_owned(), "Database error".to_owned()),
        };
        if img.owner_id != user.id {
            return ("FAILED".to_owned(), "Not enough permissions".to_owned());
        }
        if database::delete_image(&mut connection, img_id).is_err() {
            return ("FAILED".to_owned(), "Database error".to_owned());
        }
        let user_gallery = format!("users/{}/gallery", user.id);
        let dir_reader = match fs::read_dir(user_gallery) {
            Ok(dr) => dr,
            Err(_) => return ("FAILED".to_owned(), "Internal server error".to_owned()),
        };

        for path in dir_reader {
            let path = path.unwrap().path();
            if let Some(prefix) = path.file_stem() {
                let prefix = match prefix.to_str() {
                    Some(s) => s.to_string(),
                    _ => continue,
                };
                if prefix != img_id.to_string() {
                    continue;
                }
                if fs::remove_file(path).is_err() {
                    return ("FAILED".to_owned(), "Internal server error".to_owned());
                }
                break;
            }
        }
        return ("OK".to_owned(), "".to_owned());
    })();
    Ok(web::Json(json!({
        "status": status,
        "reason": fail_reason,
    })))
}

#[get("/gallery/{user_id}")] // <- define path parameters
async fn gallery_service(
    path: web::Path<u32>,
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let user_id = path.into_inner();
    let (status, fail_reason, images) = (|| {
        let mut connection = match database::try_connect(&db_config, 3) {
            Ok(conn) => conn,
            Err(_) => {
                return (
                    "FAILED".to_owned(),
                    "Failed to connect to database".to_owned(),
                    Vec::new(),
                )
            }
        };
        let user = match database::find_user_by_id(&mut connection, user_id) {
            Some(user) => user,
            None => return ("FAILED".to_owned(), "User not found".to_owned(), Vec::new()),
        };
        match database::get_images(&mut connection, user.id) {
            Ok(images) => ("OK".to_owned(), "".to_owned(), images),
            Err(_) => return ("FAILED".to_owned(), "Database error".to_owned(), Vec::new()),
        }
    })();
    Ok(web::Json(json!({
        "status": status,
        "reason": fail_reason,
        "images": images,
    })))
}

#[post("/set/{email}/{password}/{image_id}")] // <- define path parameters
async fn set_logo_service(
    path: web::Path<(String, String, u32)>,
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let (email, password, img_id) = path.into_inner();
    let (status, fail_reason) = (|| {
        let (user, mut connection) = match auth_get_user_connect(&email, &password, &db_config, 3) {
            Ok((user, connection)) => (user, connection),
            Err(err) => return ("FAILED".to_owned(), err.to_string()),
        };
        let image = match database::get_image(&mut connection, img_id) {
            Ok(Some(image)) => image,
            Ok(None) => return ("FAILED".to_owned(), "Image not found".to_owned()),
            Err(_) => return ("FAILED".to_owned(), "Database error".to_owned()),
        };
        if database::delete_logo(&mut connection, user.id).is_err() {
            return ("FAILED".to_owned(), "Database error".to_owned());
        }
        if database::set_logo(&mut connection, image.id, user.id).is_err() {
            return ("FAILED".to_owned(), "Database error".to_owned());
        }
        ("OK".to_owned(), "".to_owned())
    })();
    Ok(web::Json(json!({
        "status": status,
        "reason": fail_reason,
    })))
}

#[get("/get/{user_id}")] // <- define path parameters
async fn get_logo_service(
    path: web::Path<u32>,
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let user_id = path.into_inner();
    let (status, fail_reason, logo_id) = (|| {
        let mut connection = match database::try_connect(&db_config, 3) {
            Ok(conn) => conn,
            Err(_) => return ("FAILED".to_owned(), "Database error".to_owned(), -1),
        };
        let user = match database::find_user_by_id(&mut connection, user_id) {
            Some(user) => user,
            None => return ("FAILED".to_owned(), "User not found".to_owned(), -1),
        };
        match database::get_logo_id(&mut connection, user.id) {
            Ok(Some(id)) => ("OK".to_owned(), "".to_owned(), id as i32),
            Ok(None) => return ("FAILED".to_owned(), "Logo not found".to_owned(), -1),
            Err(_) => return ("FAILED".to_owned(), "Database error".to_owned(), -1),
        }
    })();
    Ok(web::Json(json!({
        "status": status,
        "reason": fail_reason,
        "logo_id": logo_id
    })))
}
