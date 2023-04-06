use std::io::Write;

use actix_multipart::Field;
use actix_multipart::Multipart;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use futures::{StreamExt, TryStreamExt};
use std::fs;
use std::io;
use std::path::Path;

pub async fn save_file(
    field: &mut Field,
    dir_path: &str,
    file_name: &str,
    allowed_types: &[&str],
) -> Option<()> {
    let content_type = field.content_disposition();
    let extension = Path::new(content_type.get_filename()?)
        .extension()?
        .to_str()?;
    if !allowed_types.contains(&extension) {
        return None;
    }
    let filepath = format!("{}/{}.{}", dir_path, file_name, extension);
    // File::create is blocking operation, use threadpool
    let mut f = web::block(|| std::fs::File::create(filepath))
        .await
        .ok()?
        .ok()?;

    // Field in turn is stream of *Bytes* object
    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();
        // filesystem operations are blocking, we have to use threadpool
        f = web::block(move || f.write_all(&data).map(|_| f))
            .await
            .ok()?
            .ok()?;
    }

    Some(())
}



pub async fn get_extension(
    field: &Field
) -> Option<String> {
    let content_type = field.content_disposition();
    let extension = Path::new(content_type.get_filename()?)
        .extension()?
        .to_str()?;
    Some(extension.to_owned())
}

pub async fn find_file(dir_path: &str, file_name: &str) -> Option<String> {
    let dir_path = dir_path.to_owned();
    let dir = web::block(|| fs::read_dir(dir_path)).await.ok()?.ok()?;
    for entry in dir {
        let path = entry.ok()?.path();
        if path.is_file() {
            if let Some(name) = path.file_name() {
                if name.to_string_lossy().starts_with(file_name) {
                    if let Some(ext) = path.extension() {
                        let file_name_with_ext = format!("{}.{}", file_name, ext.to_string_lossy());
                        return Some(file_name_with_ext);
                    }
                }
            }
        }
    }
    None
}
