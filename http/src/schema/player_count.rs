use actix_web::HttpResponse;

#[derive(Debug, Clone)]
pub struct PlayerCount(i64);

impl From<i64> for PlayerCount {
    fn from(value: i64) -> Self {
        PlayerCount(value)
    }
}

impl Into<HttpResponse> for PlayerCount {
    fn into(self) -> HttpResponse {
        let json_value = serde_json::value::to_value(self.0).unwrap();
        HttpResponse::Ok().json(json_value)
    }
}
