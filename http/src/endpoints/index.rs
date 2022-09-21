use actix_web::HttpRequest;

pub async fn handler(_req: HttpRequest) -> &'static str {
    "OK"
}
