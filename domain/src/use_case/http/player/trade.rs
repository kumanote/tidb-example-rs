use crate::{services, Error, Result};
use store::StoreConnection;

pub struct Params {
    pub sell_id: String,
    pub buy_id: String,
    pub amount: String,
    pub price: String,
}

pub fn execute(store_connection: &mut StoreConnection, params: Params) -> Result<bool> {
    let amount = params
        .amount
        .parse::<i32>()
        .map_err(|_| Error::new_validation_error("amount must be valid integer."))?;
    let price = params
        .price
        .parse::<i32>()
        .map_err(|_| Error::new_validation_error("amount must be valid integer."))?;
    if let Err(_) = services::player::buy_goods(
        store_connection,
        &params.sell_id,
        &params.buy_id,
        amount,
        price,
    ) {
        Ok(false)
    } else {
        Ok(true)
    }
}
