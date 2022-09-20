use crate::{Error, Result};
use store::{adapters, entities, Connection, StoreConnection};
use uuid::Uuid;

pub fn execute(store_connection: &mut StoreConnection) -> Result<()> {
    // Create a player, who has a coin and a goods.
    adapters::player::create(
        store_connection,
        entities::NewPlayer {
            id: "test",
            coins: Some(1),
            goods: Some(1),
        },
    )?;

    // Get a player.
    let test_player = adapters::player::get(store_connection, "test")?;
    println!("getPlayer: {:?}", test_player);

    // Create players with bulk inserts. Insert 1919 players totally, with 114 players per batch.
    store_connection.transaction(|conn| {
        let total = 1919;
        let batch_size = 114;
        // execute batch inserts
        let mut entities = vec![];
        let mut uuids = vec![];
        for _ in 0..total {
            let uuid = Uuid::new_v4().to_string();
            uuids.push(uuid);
        }
        for i in 0..total {
            if entities.len() >= batch_size {
                adapters::player::bulk_insert(conn, &entities)?;
                entities = vec![];
            }
            entities.push(entities::NewPlayer {
                id: uuids.get(i).unwrap(),
                coins: Some(10000),
                goods: Some(10000),
            })
        }
        if entities.len() > 0 {
            adapters::player::bulk_insert(conn, &entities)?;
        }
        Ok::<_, Error>(())
    })?;

    // Count players amount.
    let count = adapters::player::count(store_connection)?;
    println!("countPlayers: {}", count);

    // Print 3 players.
    let three_players = adapters::player::gets_by_limit(store_connection, 3)?;
    for (i, player) in three_players.into_iter().enumerate() {
        println!("print {} player: {:?}", i + 1, player);
    }
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
