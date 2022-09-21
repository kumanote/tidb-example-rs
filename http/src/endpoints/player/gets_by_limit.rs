use crate::{schema, Result};
use actix_web::{web, HttpResponse};
use domain::use_case;
use store::StorePool;

pub async fn handler(
    limit_size: web::Path<i64>,
    store_pool: web::Data<StorePool>,
) -> Result<HttpResponse> {
    let mut store_connection = store_pool.get()?;
    let limit = limit_size.into_inner();
    let logic_output =
        use_case::http::player::gets_by_limit::execute(&mut store_connection, limit)?;
    let response_body = schema::players::Players::from(logic_output);
    Ok(response_body.into())
}
