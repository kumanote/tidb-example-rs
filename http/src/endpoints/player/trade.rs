use crate::{schema, Result};
use actix_web::{web, HttpResponse};
use domain::use_case;
use serde::Deserialize;
use store::StorePool;

#[derive(Debug, Deserialize)]
pub struct TradeParams {
    sell_id: String,
    buy_id: String,
    amount: String,
    price: String,
}

impl Into<use_case::http::player::trade::Params> for TradeParams {
    fn into(self) -> use_case::http::player::trade::Params {
        use_case::http::player::trade::Params {
            sell_id: self.sell_id,
            buy_id: self.buy_id,
            amount: self.amount,
            price: self.price,
        }
    }
}

pub async fn handler(
    params: web::Form<TradeParams>,
    store_pool: web::Data<StorePool>,
) -> Result<HttpResponse> {
    let mut store_connection = store_pool.get()?;
    let web_params = params.into_inner();
    let logic_params = web_params.into();
    let logic_output = use_case::http::player::trade::execute(&mut store_connection, logic_params)?;
    let response_body = schema::player_trade_result::PlayerTradeResult::from(logic_output);
    Ok(response_body.into())
}
