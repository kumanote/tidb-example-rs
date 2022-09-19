use crate::{Result, StoreConnection, StorePool};
use diesel::r2d2::{ConnectionManager, Pool};

pub fn new_pool<S: Into<String>>(database_url: S, max_size: u32) -> Result<StorePool> {
    let manager = ConnectionManager::<StoreConnection>::new(database_url);
    Pool::builder()
        .max_size(max_size)
        .build(manager)
        .map_err(Into::into)
}
