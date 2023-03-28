use std::{fs::{File, self}, io::Write};

use crate::{
    database::{self, DBconfig},
    auth::auth_get_user_connect,
};
use actix_web::{post, web, App, HttpServer, Responder, Result as ActxResult, HttpResponse, Error, dev::Payload};
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::path::PathBuf;
use uuid::Uuid;
use actix_multipart::Multipart;
use actix_form_data::{handle_multipart, Error as FormError, Field, Form};
use futures::{Future, StreamExt};
use bytes::BytesMut;

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageData {
    pub id: u32,
    pub owner_id: u32,
    pub published_at: String,
    pub about: String,
    pub image_name: String,
    pub views: u32,
    pub likes: u32,
}

#[post("/load/{email}/{password}")]
pub async fn load_image_service(
    db_config: web::Data<DBconfig>,
    path: web::Path<(String, String)>,
    image_data: web::Query<ImageData>,
) -> Result<impl Responder, Error> {

    Ok(web::Json(json!({
        "status": "image upload successful",
        "reason": ""
    })))
}

async fn image_upload_local_request(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut file_sizes = String::new();
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_type = field.content_disposition();
        let file_name = content_type.get_filename().unwrap();
        if file_name == "" {
            break;
        }
        let file_path = format!("{}", file_name);
        let mut file = web::block(|| std::fs::File::create(file_path))
            .await
            .unwrap();
        let mut file_size: usize = 0;
        let mut file_bytes = BytesMut::new();
        while let Some(chunk) = field.next().await {
            let data = chunk?;
            file_bytes.extend_from_slice(&data);
            file_size += data.len();
            file = web::block(move || file.unwrap().write_all(&data).map(|_| file)).await.unwrap().unwrap();
        }
        let thumb_size = image_create_preview(file_bytes, file_name.to_owned()).await?;
        file_sizes.push_str(&format!(
            "\"{}\": {{ \"full\": {}, \"thumb\": {} }}, ",
            file_name, file_size, thumb_size
        ));
    }
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(format!("{{ {} }}", &file_sizes[..file_sizes.len() - 2]))
        .into())
}
async fn image_create_preview(image_bytes: BytesMut, file_name: String) -> Result<usize, Error> {
    let img = image::load_from_memory(&image_bytes).unwrap();
    let scaled = img.thumbnail(100, 100);
    let file_path = format!("{}", file_name);
    let file_path_clone = file_path.clone();
    web::block(move || scaled.save(&file_path_clone)).await?;
    let file_size: usize = fs::metadata(&file_path)?.len() as usize;
    Ok(file_size)
}