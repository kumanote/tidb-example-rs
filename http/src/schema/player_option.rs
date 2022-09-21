use super::player::Player;
use actix_web::HttpResponse;
use serde::Serialize;
use store::entities;

#[derive(Debug, Clone, Serialize)]
pub struct PlayerOption(Option<Player>);

impl From<Option<entities::Player>> for PlayerOption {
    fn from(value: Option<entities::Player>) -> Self {
        Self(value.map(Into::into))
    }
}

impl Into<HttpResponse> for PlayerOption {
    fn into(self) -> HttpResponse {
        let json_value = serde_json::value::to_value(self.0).unwrap();
        HttpResponse::Ok().json(json_value)
    }
}
