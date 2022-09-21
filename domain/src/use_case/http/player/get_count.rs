use crate::Result;
use store::{adapters, StoreConnection};

pub fn execute(store_connection: &mut StoreConnection) -> Result<i64> {
    adapters::player::count(store_connection).map_err(Into::into)
}
