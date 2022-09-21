use super::player::Player;
use actix_web::HttpResponse;
use serde::Serialize;
use store::entities;

#[derive(Debug, Clone, Serialize)]
pub struct Players(Vec<Player>);

impl From<Vec<entities::Player>> for Players {
    fn from(values: Vec<entities::Player>) -> Self {
        Self(values.into_iter().map(Into::into).collect())
    }
}

impl Into<HttpResponse> for Players {
    fn into(self) -> HttpResponse {
        let json_value = serde_json::value::to_value(self.0).unwrap();
        HttpResponse::Ok().json(json_value)
    }
}
