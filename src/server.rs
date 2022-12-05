use actix_web::{
    error, get,
    http::header::{self, ContentType},
    post, web, Error, HttpRequest, HttpResponse,
};
use futures_util::TryStreamExt;

use crate::{config::Config, paste::Paste, DEFAULT_MIME_TYPE, LANDING_PAGE};

#[get("/")]
async fn index(req: HttpRequest) -> HttpResponse {
    let hostname = hostname(req);
    let template = LANDING_PAGE;
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

#[post("/")]
async fn upload(
    req: HttpRequest,
    mut payload: web::Payload,
    config: web::Data<Config>,
) -> Result<HttpResponse, Error> {
    let mut bytes = web::BytesMut::new();
    while let Some(item) = payload.try_next().await? {
        bytes.extend_from_slice(&item);
        if bytes.len() > config.max_content_length {
            return Err(error::ErrorPayloadTooLarge("upload limit exceed"));
        }
    }

    if bytes.is_empty() {
        return Err(error::ErrorBadRequest("invalid file"));
    }

    let paste = Paste {
        data: bytes.to_vec(),
    };

    let mut upload_path = config.upload_path.clone();
    match paste.save_to(&mut upload_path) {
        Ok(path) => {
            if let Some(file_stem) = path.file_stem() {
                let location = format!("{}/{}", hostname(req), file_stem.to_str().unwrap());
                let mut body = location.clone();
                body.push('\n');
                log::debug!("location: {}", &location);
                Ok(HttpResponse::Found()
                    .insert_header((header::LOCATION, location))
                    .body(body))
            } else {
                Err(error::ErrorInternalServerError("paste fail"))
            }
        }
        Err(e) => {
            log::error!("paste fail: {e}");
            Err(error::ErrorInternalServerError(e))
        }
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(index).service(upload).service(serve);
}

fn hostname(req: HttpRequest) -> String {
    const FORWARDED_HEADER_PREFIX: &str = "x-forwarded-";

    let headers = req.headers();
    let mut host: Vec<&str> = Vec::new();
    for word in ["protocol", "host"] {
        match headers.get(format!("{}{}", FORWARDED_HEADER_PREFIX, word)) {
            Some(header_value) => match header_value.to_str() {
                Ok(value) => {
                    host.push(value);
                }
                Err(e) => {
                    log::trace!("unexpected header value: {e}");
                    host.clear();
                    break;
                }
            },
            None => {
                host.clear();
                break;
            }
        }
    }

    if host.is_empty() {
        format!(
            "http://{}",
            headers.get(header::HOST).unwrap().to_str().unwrap()
        )
    } else {
        host.join("://")
    }
}
