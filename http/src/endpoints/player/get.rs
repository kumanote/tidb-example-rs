use crate::{schema, Result};
use actix_web::{web, HttpResponse};
use domain::use_case;
use store::StorePool;

pub async fn handler(
    id: web::Path<String>,
    store_pool: web::Data<StorePool>,
) -> Result<HttpResponse> {
    let mut store_connection = store_pool.get()?;
    let id = id.into_inner();
    let logic_output = use_case::http::player::get::execute(&mut store_connection, id)?;
    let response_body = schema::player_option::PlayerOption::from(logic_output);
    Ok(response_body.into())
}
