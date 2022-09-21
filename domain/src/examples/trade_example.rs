use crate::{services, Result};
use store::{adapters, entities, StoreConnection};

pub fn execute(store_connection: &mut StoreConnection) -> Result<()> {
    // Player 1: id is "1", has only 100 coins.
    // Player 2: id is "2", has 114514 coins, and 20 goods.
    let player1_id = "1";
    let player2_id = "2";
    adapters::player::create(
        store_connection,
        entities::NewPlayer {
            id: player1_id,
            coins: Some(100),
            goods: None,
        },
    )?;
    adapters::player::create(
        store_connection,
        entities::NewPlayer {
            id: player2_id,
            coins: Some(114514),
            goods: Some(20),
        },
    )?;
    println!("\nbuyGoods:\n    => this trade will fail");
    assert!(
        services::player::buy_goods(store_connection, player2_id, player1_id, 10, 500).is_err()
    );

    println!("\nbuyGoods:\n    => this trade will success");
    services::player::buy_goods(store_connection, player2_id, player1_id, 2, 100)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[serial_test::serial]
    fn test_execute() {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or("mysql://root:@127.0.0.1:4000/test?charset=utf8mb4".to_owned());
        let mut conn = store::establish_connection(database_url).unwrap();
        adapters::player::truncate(&mut conn).unwrap();
        let ok = execute(&mut conn).is_ok();
        assert!(ok)
    }
}
