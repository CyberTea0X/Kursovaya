use crate::database::{self, DBconfig, User};
use actix_web::{
    dev::ConnectionInfo, get, post, web, App, HttpResponse, HttpServer, Responder,
    Result as ActxResult,
};
use serde::Deserialize;
use serde_json::json;

#[post("/popular")]
pub async fn search_popular(db_config: web::Data<DBconfig>) -> ActxResult<impl Responder> {
    let (status, fail_reason, items) = (|| {
        let connection = database::try_connect(&db_config, 3);
        if connection.is_err() {
            println!("Failed to connect to database");
            return (
                "FAILED".to_string(),
                "Failed to connect to database".to_string(),
                Vec::new(),
            );
        }
        let mut connection = connection.unwrap();
        let items = database::search_popular(&mut connection);
        if items.is_err() {
            return (
                "FAILED".to_string(),
                "Database error".to_string(),
                Vec::new(),
            );
        }
        return ("OK".to_string(), "".to_string(), items.unwrap());
    })();
    Ok(web::Json(json!({
        "items": items,
        "status": status,
        "reason": fail_reason,
    })))
}

#[post("/login/{login}")]
pub async fn search_login(
    db_config: web::Data<DBconfig>,
    path: web::Path<String>
) -> ActxResult<impl Responder> {
    let (status, fail_reason, items) = (|| {
        let login = path.into_inner();
        let connection = database::try_connect(&db_config, 3);
        if connection.is_err() {
            println!("Failed to connect to database");
            return (
                "FAILED".to_string(),
                "Failed to connect to database".to_string(),
                Vec::new(),
            );
        }
        let mut connection = connection.unwrap();
        let users = database::search_login(&mut connection, &login);
        if users.is_err() {
            return (
                "FAILED".to_string(),
                "Database error".to_string(),
                Vec::new(),
            );
        }
        return ("OK".to_string(), "".to_string(), users.unwrap());
    })();
    Ok(web::Json(json!({
        "items": items,
        "status": status,
        "reason": fail_reason,
    })))
}

#[post("/name")]
pub async fn search_name(db_config: web::Data<DBconfig>) -> ActxResult<impl Responder> {
    let (status, fail_reason, items) = (|| {
        // TODO: Поиск по имени и/или фамили
        return ("OK".to_string(), "".to_string(), Vec::<User>::new());
    })();
    Ok(web::Json(json!({
        "items": items,
        "status": status,
        "reason": fail_reason,
    })))
}

#[post("/mail")]
pub async fn search_mail(db_config: web::Data<DBconfig>) -> ActxResult<impl Responder> {
    let (status, fail_reason, items) = (|| {
        // TODO: Поиск по почте (наиболее похожие)
        return ("OK".to_string(), "".to_string(), Vec::<User>::new());
    })();
    Ok(web::Json(json!({
        "items": items,
        "status": status,
        "reason": fail_reason,
    })))
}
