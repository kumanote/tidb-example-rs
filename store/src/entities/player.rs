use crate::schema::player;

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = player)]
pub struct Player {
    pub id: String,
    pub coins: Option<i32>,
    pub goods: Option<i32>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = player)]
pub struct NewPlayer<'a> {
    pub id: &'a str,
    pub coins: Option<i32>,
    pub goods: Option<i32>,
}
