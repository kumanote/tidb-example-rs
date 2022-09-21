use crate::Result;
use store::{adapters, entities, StoreConnection};

pub fn execute(
    store_connection: &mut StoreConnection,
    limit: i64,
) -> Result<Vec<entities::Player>> {
    adapters::player::gets_by_limit(store_connection, limit).map_err(Into::into)
}
