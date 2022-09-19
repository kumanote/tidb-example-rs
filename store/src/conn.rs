use crate::{Result, StoreConnection};
use diesel::prelude::*;

pub fn establish_connection<S: Into<String>>(database_url: S) -> Result<StoreConnection> {
    let database_url = database_url.into();
    Ok(StoreConnection::establish(&database_url)?)
}
