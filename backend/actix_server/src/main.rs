use actix_files as fs;

use actix_web::{get, post, web, App, HttpServer, Responder, Result as ActxResult};
use database::DBconfig;
use serde_json::json;
use std::fs::{create_dir, metadata};

pub mod auth;
pub mod chat;
pub mod claims;
pub mod database;
pub mod email;
pub mod image;
pub mod jwt;
pub mod messages;
pub mod passwords;
pub mod register;
pub mod search;
pub mod user;

fn init_users_dir() {
    let dir_name = "users";
    if !metadata(dir_name).is_ok() {
        create_dir(dir_name).unwrap();
    }
}

#[post("/login/{email}/{password}")] // <- define path parameters
async fn login_service(
    path: web::Path<(String, String)>,
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let (email, password) = path.into_inner();

    let (status, fail_reason) = (|| {
        return match auth::auth_get_user_connect(&email, &password, &db_config, 3) {
            Ok(_) => ("OK".to_owned(), "".to_owned()),
            Err(err) => ("Failed".to_owned(), err.to_string()),
        };
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
    init_users_dir();
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(database::parse_config()))
            .service(
                web::scope("api/images")
                    .service(image::load_image_service)
                    .service(fs::Files::new("/", "./users/.").show_files_listing()),
            )
            .service(
                web::scope("api/chat")
                    .service(chat::create_chat_service)
                    .service(chat::delete_chat_service)
                    .service(chat::get_user_chats_service)
                    .service(chat::is_chat_exists_service),
            )
            .service(
                web::scope("api/messages")
                    .service(messages::send_message_service)
                    .service(messages::get_messages_service)
                    .service(messages::read_messages_service),
            )
            .service(
                web::scope("/api/search")
                    .service(search::search_login_service)
                    .service(search::search_popular_service)
                    .service(search::search_name_service)
                    .service(search::search_text_service),
            )
            .service(
                web::scope("/api")
                    .service(login_service)
                    .service(register::register_user_service)
                    .service(user::user_profile_service)
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
