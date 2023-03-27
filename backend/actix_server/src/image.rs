use crate::database::{self, DBconfig};
use actix_web::{post, web, App, HttpServer, Responder, Result as ActxResult};
use serde_json::json;

#[post("/load")]
pub async fn load_image_service(db_config: web::Data<DBconfig>) -> ActxResult<impl Responder> {
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
