use actix_web::HttpResponse;
use serde::Serialize;
use store::entities;

#[derive(Debug, Clone, Serialize)]
pub struct Player {
    pub id: String,
    pub coins: i32,
    pub goods: i32,
}

impl From<entities::Player> for Player {
    fn from(value: entities::Player) -> Self {
        Self {
            id: value.id,
            coins: value.coins.unwrap_or(0),
            goods: value.goods.unwrap_or(0),
        }
    }
}

impl Into<HttpResponse> for Player {
    fn into(self) -> HttpResponse {
        let json_value = serde_json::value::to_value(self).unwrap();
        HttpResponse::Ok().json(json_value)
    }
}
