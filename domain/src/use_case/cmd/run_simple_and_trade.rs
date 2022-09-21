use crate::{examples, Result};
use store::StoreConnection;

pub fn execute(store_connection: &mut StoreConnection) -> Result<()> {
    examples::simple_example::execute(store_connection)?;
    examples::trade_example::execute(store_connection)?;
    Ok(())
}
