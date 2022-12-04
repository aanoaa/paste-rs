use actix_web::{
    get,
    http::header::{self, ContentType},
    web, HttpRequest, HttpResponse,
};

use crate::LANDING_PAGE;

#[get("/")]
async fn index(req: HttpRequest) -> HttpResponse {
    let headers = req.headers();
    let host = headers.get(header::HOST).unwrap().to_str().unwrap();
    let template = LANDING_PAGE;
    let content = template.replace(":HOST:", host);
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(content)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}
