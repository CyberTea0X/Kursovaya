use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result as ActxResult};
use database::DBconfig;
use serde_json::json;

mod database;
mod passwords;

#[post("/login/{email}/{password}")] // <- define path parameters
async fn login_service(
    path: web::Path<(String, String)>,
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let (email, password) = path.into_inner();

    let (status, fail_reason) = (|| {
        if !passwords::is_valid_password(&password) {
            return ("FAILED", "Password contains invalid characters or too small");
        }
        let connection = database::try_connect(&db_config, 3);
        if connection.is_err() {
            println!("Failed to connect to database");
            return ("FAILED", "Failed to connect to database");
        }
        let mut connection = connection.unwrap();
        let user = database::find_user(&mut connection, &email);
        if user.is_none() {
            return ("FAILED", "User does not exist");
        }
        let user = user.unwrap();
        if user.password == password {
            return ("OK", "");
        }
        else {
            return ("FAILED", "Invalid password");
        }
    })();
    Ok(web::Json(json!({
        "status": status,
        "reason": fail_reason,
    })))
}

#[post("/registration/{first_name}/{last_name}/{password}/{email}/{logo}/{about}")]
async fn register(
    path: web::Path<(String, String, String, String, String, String)>,
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let (first_name, last_name, password, email, logo, about) = path.into_inner();
    let (status, fail_reason) = (||{
        if !passwords::is_valid_password(&password) {
            return  ("FAILED", "Password contains invalid characters or too small")
        }
        let connection = database::try_connect(&db_config, 3);
        if connection.is_err() {
            println!("Failed to connect to database");
            return ("FAILED", "Failed to connect to database");
        }
        let mut connection = connection.unwrap();
        if database::user_exists(&mut connection, &email) {
            return ("FAILED", "User already registered");
        }
        if database::register_user(
            &mut connection,
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

#[post("/profile/{email}")] // <- define path parameters
async fn fetch_user_profile(
    path: web::Path<String>,
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let email = path.into_inner();
    let (status, fail_reason, user) = (||{
        let connection = database::try_connect(&db_config, 3);
        if connection.is_err() {
            println!("Failed to connect to database");
            return ("FAILED", "Failed to connect to database", database::User::default());
        }
        let mut connection = connection.unwrap();
        let user = database::find_user(&mut connection, &email);
        if user.is_none() {
            return ("FAILED", "User does not exist", database::User::default());
        }
        let user = user.unwrap();
        return ("OK", "", user);
    })();
    Ok(web::Json(json!({
        "status": status,
        "reason": fail_reason,
        "user": user,
    })))
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
            .app_data(web::Data::new(database::parse_config()))
            .service(login_service)
            .service(register)
            .service(fetch_user_profile)
            .service(config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
