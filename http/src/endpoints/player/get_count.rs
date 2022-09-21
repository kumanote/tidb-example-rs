use crate::{schema, Result};
use actix_web::{web, HttpResponse};
use domain::use_case;
use store::StorePool;

pub async fn handler(store_pool: web::Data<StorePool>) -> Result<HttpResponse> {
    let mut store_connection = store_pool.get()?;
    let logic_output = use_case::http::player::get_count::execute(&mut store_connection)?;
    let response_body = schema::player_count::PlayerCount::from(logic_output);
    Ok(response_body.into())
}
