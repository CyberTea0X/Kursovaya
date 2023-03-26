use actix_web::{get, post, web, App, HttpServer, Responder, Result as ActxResult};
use crate::database::{self, DBconfig};
use serde_json::json;
use actix_files::NamedFile;

#[get("/get/{userid}/{filename}")]
pub async fn get_image_service(
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    // let connection = match database::try_connect(&db_config, 3) {
    //     Ok(conn) => conn,
    //     Err(_) => return Err("Failed to connect to database".to_owned()),
    // };
    // let user = database::find_user(connection, email, id, hide_password)
    Ok("some image")
    // let path = format!("images/{}.jpg", id);
    // let mut file = File::open(path)?;
    // let mut buffer = Vec::new();
    // file.read_to_end(&mut buffer)?;
    // Ok(HttpResponse::Ok().content_type("image/jpeg").body(buffer))
    // return ("OK".to_owned(), "".to_owned());
}

#[post("/load")]
pub async fn load_image_service(
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let (status, fail_reason) = (|| {
        return ("OK".to_owned(), "".to_owned());
    })();
    Ok(web::Json(json!({
        "status": status,
        "reason": fail_reason,
    })))
}

// pub fn error_image() -> Result<NamedFile, > {
//     Ok(NamedFile::open("path/to/image.png")?)
// }