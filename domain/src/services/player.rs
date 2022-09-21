use crate::{Error, Result};
use store::{adapters, StoreConnection};

pub fn buy_goods(
    store_connection: &mut StoreConnection,
    sell_id: &str,
    buy_id: &str,
    amount: i32,
    price: i32,
) -> Result<()> {
    let sell_player = adapters::player::select_for_update(store_connection, sell_id)?;
    if sell_player.goods.unwrap_or(0) < amount {
        return Err(Error::new_validation_error(format!(
            "sell player {} goods not enough",
            sell_id
        )));
    }
    let buy_player = adapters::player::select_for_update(store_connection, buy_id)?;
    if buy_player.coins.unwrap_or(0) < price {
        return Err(Error::new_validation_error(format!(
            "buy player {} coins not enough",
            buy_id
        )));
    }
    let updated = adapters::player::update(
        store_connection,
        sell_player.coins.unwrap_or(0) + price,
        sell_player.goods.unwrap_or(0) - amount,
        &sell_player.id,
    )?;
    assert_eq!(updated, 1);
    let updated = adapters::player::update(
        store_connection,
        buy_player.coins.unwrap_or(0) - price,
        buy_player.goods.unwrap_or(0) + amount,
        &buy_player.id,
    )?;
    assert_eq!(updated, 1);
    println!("\n[buyGoods]:\n    'trade success'");
    Ok(())
}
