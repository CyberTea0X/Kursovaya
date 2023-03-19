use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result as ActxResult};
use database::DBconfig;
use mysql::Conn;
use serde_json::json;

mod database;
mod passwords;

#[post("/login/{login}/{password}")] // <- define path parameters
async fn login_service(path: web::Path<(String, String)>) -> ActxResult<String> {
    let (login, password) = path.into_inner();
    let responce = format!("Trying to login {}, password {}", login, password);
    Ok(responce)
}

#[post("/registration/{first_name}/{last_name}/{password}/{email}/{logo}/{about}")]
async fn register(
    path: web::Path<(String, String, String, String, String, String)>,
    connection: web::Data<database::DBconnection>,
) -> ActxResult<impl Responder> {
    let (first_name, last_name, password, email, logo, about) = path.into_inner();
    let (status, fail_reason) = (||{
        if !passwords::is_valid_password(&password) {
            return  ("FAILED", "Password contains invalid characters or too small")
        }
        if !connection.lock().unwrap().ping() {
            if !database::try_reconnect(&mut connection.lock().unwrap()) {
                return ("FAILED", "Failed to connect to database");
            }
        }
        if database::is_registered(&mut connection.lock().unwrap(), &email) {
            return ("FAILED", "User already registered");
        }
        if database::register_user(
            &mut connection.lock().unwrap(),
            &first_name,
            &last_name,
            &password,
            &email,
            &logo,
            &about,
        ) {
            return ("OK", "")
        } else {
            return ("FAILED", "Database error")
        };
    })();
    Ok(web::Json(json!({
        "status": status,
        "reason": fail_reason,
    })))
}

#[post("/profile/{user_id}")] // <- define path parameters
async fn fetch_user_profile(path: web::Path<u32>) -> ActxResult<String> {
    let user_id = path.into_inner();
    let responce = format!("Fectching profile from Database {}", user_id);
    Ok(responce)
}

#[post("/config")] // <- define path parameters
async fn config(db_config: web::Data<DBconfig>) -> ActxResult<String> {
    let responce = format!(
        "ip: {}\nport: {}\nuser: {}\npassword: {},\ndatabase: {}",
        db_config.ip, db_config.port, db_config.user, db_config.password, db_config.database
    );
    Ok(responce)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(
                loop {
                    let con = database::connect_to_db(database::parse_config());
                    if con.is_ok() {
                        break con.unwrap();
                    }
                    else {
                        println!("Failed to connect to database, retrying...");
                    }
                }
            ))
            .service(login_service)
            .service(register)
            .service(fetch_user_profile)
            .service(config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
