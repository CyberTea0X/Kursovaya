use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result as ActxResult};
use serde::Deserialize;
use crate::database::{DBconfig, User};
use serde_json::json;


#[post("/popular")]
pub async fn search_popular(
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let (status, fail_reason, items) = (|| {
        // TODO: Поиск по рейтингу (самые популярные)
        return ("OK".to_string(), "".to_string(), Vec::<User>::new());
    })();
    Ok(web::Json(json!({
        "items": items,
        "status": status,
        "reason": fail_reason,
    })))
}

#[post("/login")]
pub async fn search_login(
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let (status, fail_reason, items) = (|| {
        // TODO: Поиск по логину (наиболее похожие)
        return ("OK".to_string(), "".to_string(), Vec::<User>::new());
    })();
    Ok(web::Json(json!({
        "items": items,
        "status": status,
        "reason": fail_reason,
    })))
}

#[post("/name")]
pub async fn search_name(
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
    let (status, fail_reason, items) = (|| {
        // TODO: Поиск по имени и/или фамилии
        return ("OK".to_string(), "".to_string(), Vec::<User>::new());
    })();
    Ok(web::Json(json!({
        "items": items,
        "status": status,
        "reason": fail_reason,
    })))
}

#[post("/mail")]
pub async fn search_mail(
    db_config: web::Data<DBconfig>,
) -> ActxResult<impl Responder> {
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
