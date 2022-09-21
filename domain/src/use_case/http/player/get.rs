use crate::Result;
use store::{adapters, entities, StoreConnection};

pub fn execute(
    store_connection: &mut StoreConnection,
    id: String,
) -> Result<Option<entities::Player>> {
    adapters::player::get(store_connection, &id).map_err(Into::into)
}
