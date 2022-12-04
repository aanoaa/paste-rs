use actix_web::{
    get,
    http::header::{self, ContentType},
    web, HttpRequest, HttpResponse,
};

use crate::LANDING_PAGE;

#[get("/")]
async fn index(req: HttpRequest) -> HttpResponse {
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

    let hostname = if host.is_empty() {
        String::from(headers.get(header::HOST).unwrap().to_str().unwrap())
    } else {
        host.join("://")
    };

    let template = LANDING_PAGE;
    let content = template.replace(":HOST:", &hostname);
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(content)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}
