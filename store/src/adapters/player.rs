use crate::entities::{NewPlayer, Player};
use crate::schema::player;
use crate::{Result, StoreConnection};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};

pub fn truncate(conn: &mut StoreConnection) -> Result<()> {
    let query = "TRUNCATE TABLE player";
    diesel::sql_query(query).execute(conn)?;
    Ok(())
}

pub fn create(conn: &mut StoreConnection, entity: NewPlayer) -> Result<usize> {
    diesel::insert_into(player::table)
        .values(&entity)
        .execute(conn)
        .map_err(Into::into)
}

pub fn bulk_insert(conn: &mut StoreConnection, entities: &Vec<NewPlayer>) -> Result<usize> {
    diesel::insert_into(player::table)
        .values(entities)
        .execute(conn)
        .map_err(Into::into)
}

pub fn update(conn: &mut StoreConnection, coins: i32, goods: i32, id: &str) -> Result<usize> {
    diesel::update(player::dsl::player.find(id))
        .set((player::coins.eq(coins), player::goods.eq(goods)))
        .execute(conn)
        .map_err(Into::into)
}

pub fn get(conn: &mut StoreConnection, id: &str) -> Result<Option<Player>> {
    let result = player::table.find(id).first::<Player>(conn);
    match result {
        Ok(entity) => Ok(Some(entity)),
        Err(err) => match err {
            Error::NotFound => Ok(None),
            _ => Err(err.into()),
        },
    }
}

pub fn count(conn: &mut StoreConnection) -> Result<i64> {
    player::table.count().get_result(conn).map_err(Into::into)
}

pub fn select_for_update(conn: &mut StoreConnection, id: &str) -> Result<Player> {
    player::table
        .find(id)
        .for_update()
        .first::<Player>(conn)
        .map_err(Into::into)
}

pub fn gets_by_limit(conn: &mut StoreConnection, limit: i64) -> Result<Vec<Player>> {
    player::table
        .limit(limit)
        .load::<Player>(conn)
        .map_err(Into::into)
}
