use std::{fs, path::PathBuf};

use actix_web::{
    delete, error, get,
    http::header::{self, ContentType},
    post, web, Error, HttpRequest, HttpResponse,
};
use futures_util::TryStreamExt;

use crate::{config::Config, paste::Paste, DEFAULT_MIME_TYPE, LANDING_PAGE};

#[get("/")]
async fn index(req: HttpRequest) -> HttpResponse {
    let template = LANDING_PAGE;
    let info = req.connection_info();
    let hostname = format!("{}://{}", info.scheme(), info.host());
    let content = template.replace(":HOST:", &hostname);
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(content)
}

#[get("/{id}")]
async fn serve(id: web::Path<String>, config: web::Data<Config>) -> Result<HttpResponse, Error> {
    let mut upload_path = config.upload_path.clone();
    upload_path.push(&*id);
    let paste = Paste::from(&upload_path)?;
    Ok(HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, DEFAULT_MIME_TYPE))
        .body(paste.data))
}

#[delete("/{id}")]
async fn delete(id: web::Path<String>, config: web::Data<Config>) -> Result<HttpResponse, Error> {
    let mut path = config.upload_path.clone();
    path.push(&*id);
    if !path.exists() {
        Err(error::ErrorNotFound("file not found"))
    } else {
        fs::remove_file(&path)?;
        log::info!("file deleted successfully: {:?}", &path);
        Ok(HttpResponse::Ok().body("OK\n"))
    }
}

#[post("/")]
async fn upload(
    req: HttpRequest,
    mut payload: web::Payload,
    config: web::Data<Config>,
) -> Result<HttpResponse, Error> {
    let mut content_len: usize = 0;
    let file_name = Paste::random_file_name(config.upload_path.to_str());
    let upload_path = PathBuf::from(file_name);
    while let Some(chunk) = payload.try_next().await? {
        let mut bytes = web::BytesMut::new();
        bytes.extend_from_slice(&chunk);
        content_len += bytes.len();
        if content_len > config.max_content_length {
            if upload_path.exists() {
                fs::remove_file(upload_path)?;
            }
            return Err(error::ErrorPayloadTooLarge("upload limit exceed"));
        }
        let appended_len = Paste::append(&mut bytes.to_vec(), &upload_path)?;
        log::trace!("file appended {} length", appended_len);
    }

    if content_len == 0 {
        return Err(error::ErrorBadRequest("empty file not allowed"));
    }

    log::info!("file uploaded successfully: {:?}", &upload_path);
    if let Some(file_stem) = upload_path.file_stem() {
        let info = req.connection_info();
        let hostname = format!("{}://{}", info.scheme(), info.host());
        let location = format!("{}/{}", hostname, file_stem.to_str().unwrap());
        let mut body = location.clone();
        body.push('\n');
        Ok(HttpResponse::Found()
            .insert_header((header::LOCATION, location))
            .body(body))
    } else {
        Err(error::ErrorInternalServerError("paste fail"))
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(upload)
        .service(serve)
        .service(delete);
}
