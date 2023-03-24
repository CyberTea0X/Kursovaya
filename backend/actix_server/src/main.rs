use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result as ActxResult};
use database::DBconfig;
use serde_json::json;

pub mod claims;
pub mod database;
pub mod email;
pub mod jwt;
pub mod passwords;
pub mod register;
pub mod search;
pub mod user;

#[post("/login/{email}/{password}")] // <- define path parameters
async fn login_service(
    path: web::Path<(String, String)>,
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let (email, password) = path.into_inner();

    let (status, fail_reason) = (|| {
        if !email::is_valid_email(email.as_str()) {
            return ("FAILED".to_string(), "Invalid email adresss".to_string());
        }
        if !passwords::is_valid_password(&password) {
            return (
                "FAILED".to_string(),
                "Password contains invalid characters or too small".to_string(),
            );
        }
        let connection = database::try_connect(&db_config, 3);
        if connection.is_err() {
            println!("Failed to connect to database");
            return (
                "FAILED".to_string(),
                "Failed to connect to database".to_string(),
            );
        }
        let mut connection = connection.unwrap();
        return passwords::check_password(&mut connection, &email, &password);
    })();
    Ok(web::Json(json!({
        "status": status,
        "reason": fail_reason,
    })))
}

#[get("/dbstatus")] //
async fn check_db_status(db_config: web::Data<DBconfig>) -> String {
    let connection = database::try_connect(&db_config, 3);
    format!(
        "DB {}",
        if connection.is_ok() {
            "Online"
        } else {
            "Offline"
        }
    )
}

// #[post("/config")] // <- define path parameters
// async fn config(db_config: web::Data<DBconfig>) -> ActxResult<String> {
//     let responce = format!(
//         "ip: {}\nport: {}\nuser: {}\npassword: {},\ndatabase: {}",
//         db_config.ip, db_config.port, db_config.user, db_config.password, db_config.database
//     );
//     Ok(responce)
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(database::parse_config()))
            .service(
                web::scope("/api/search")
                    .service(search::search_login)
                    .service(search::search_popular),
            )
            .service(
                web::scope("/api")
                    .service(login_service)
                    .service(register::register_service)
                    .service(user::fetch_user_profile)
                    .service(check_db_status)
                    .service(user::edit_user_service)
                    .service(user::delete_user_service)
                    .service(user::visit_user_service),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
