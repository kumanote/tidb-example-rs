use actix_web::HttpResponse;

#[derive(Debug, Clone)]
pub struct PlayerTradeResult(bool);

impl From<bool> for PlayerTradeResult {
    fn from(value: bool) -> Self {
        PlayerTradeResult(value)
    }
}

impl Into<HttpResponse> for PlayerTradeResult {
    fn into(self) -> HttpResponse {
        let json_value = serde_json::value::to_value(self.0).unwrap();
        HttpResponse::Ok().json(json_value)
    }
}
