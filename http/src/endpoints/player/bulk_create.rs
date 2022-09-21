use crate::{schema, Result};
use actix_web::{web, HttpResponse};
use domain::use_case;
use serde::Deserialize;
use store::StorePool;

#[derive(Debug, Deserialize)]
pub struct CreatePlayer {
    coins: Option<i32>,
    goods: Option<i32>,
}

impl Into<use_case::http::player::bulk_create::CreatePlayer> for CreatePlayer {
    fn into(self) -> use_case::http::player::bulk_create::CreatePlayer {
        use_case::http::player::bulk_create::CreatePlayer {
            coins: self.coins,
            goods: self.goods,
        }
    }
}

pub async fn handler(
    params: web::Json<Vec<CreatePlayer>>,
    store_pool: web::Data<StorePool>,
) -> Result<HttpResponse> {
    let mut store_connection = store_pool.get()?;
    let web_params = params.into_inner();
    let logic_params = web_params.into_iter().map(Into::into).collect();
    let logic_output =
        use_case::http::player::bulk_create::execute(&mut store_connection, logic_params)?;
    let response_body =
        schema::player_bulk_create_result::PlayerBulkCreateResult::from(logic_output);
    Ok(response_body.into())
}
