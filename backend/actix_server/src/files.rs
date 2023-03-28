use std::io::Write;

use actix_multipart::Multipart;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use futures::{StreamExt, TryStreamExt};
use std::path::Path;

pub async fn save_file(
    mut payload: Multipart,
    dir_path: &str,
    file_name: &str,
    allowed_types: &[&str],
) -> Option<()> {
    // iterate over multipart stream
    let mut field = match payload.try_next().await {
        Ok(Some(field)) => field,
        _ => return None,
    };
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
