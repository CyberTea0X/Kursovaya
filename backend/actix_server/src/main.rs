use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result as ActxResult};
use database::DBconfig;

mod database;

#[post("/login/{login}/{password}")] // <- define path parameters
async fn login_service(path: web::Path<(String, String)>) -> ActxResult<String> {
    let (login, password) = path.into_inner();
    let responce = format!("Trying to login {}, password {}", login, password);
    Ok(responce)
}

#[post("/registration/{first_name}/{last_name}/{password}/{email}/{logo}/{about}")]
async fn register(
    path: web::Path<(String, String, String, String, String, String)>,
    db_config: web::Data<DBconfig>,
) -> ActxResult<String> {
    let (first_name, last_name, password, email, logo, about) = path.into_inner();
    let db_config = db_config.get_ref().clone();
    let status = if database::register_user(
        db_config,
        &first_name,
        &last_name,
        &password,
        &email,
        &logo,
        &about,
    ) {
        "OK"
    } else {
        "FAILED"
    };
    let responce = format!(
        "Register name: {}\nsurname: {}\npassword: {}\n\
    email: {}\nlogo: {}\nabout: {}\nstatus: {}",
        first_name, last_name, password, email, logo, about, status
    );
    Ok(responce)
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
