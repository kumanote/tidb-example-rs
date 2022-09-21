use actix_web::HttpResponse;

#[derive(Debug, Clone)]
pub struct PlayerBulkCreateResult(usize);

impl From<usize> for PlayerBulkCreateResult {
    fn from(value: usize) -> Self {
        PlayerBulkCreateResult(value)
    }
}

impl Into<HttpResponse> for PlayerBulkCreateResult {
    fn into(self) -> HttpResponse {
        let json_value = serde_json::value::to_value(self.0).unwrap();
        HttpResponse::Ok().json(json_value)
    }
}
