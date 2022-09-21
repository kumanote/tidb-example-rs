use crate::Result;
use store::{adapters, entities, StoreConnection};
use uuid::Uuid;

pub type Params = Vec<CreatePlayer>;

pub struct CreatePlayer {
    pub coins: Option<i32>,
    pub goods: Option<i32>,
}

pub fn execute(store_connection: &mut StoreConnection, params: Params) -> Result<usize> {
    let batch_size = 100;
    let mut entities = vec![];
    let uuids: Vec<String> = params.iter().map(|_| Uuid::new_v4().to_string()).collect();
    for (i, item) in params.iter().enumerate() {
        if entities.len() >= batch_size {
            adapters::player::bulk_insert(store_connection, &entities)?;
            entities = vec![];
        }
        entities.push(entities::NewPlayer {
            id: uuids.get(i).unwrap(),
            coins: item.coins,
            goods: item.goods,
        })
    }
    if entities.len() > 0 {
        adapters::player::bulk_insert(store_connection, &entities)?;
    }
    Ok(params.len())
}
