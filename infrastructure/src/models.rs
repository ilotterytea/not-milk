use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Consumer {
    pub id: i32,
    pub alias_id: i32,
    pub alias_name: String,
    pub alias_pfp: String,
    pub is_suspended: i32,
    pub created_at: i32,
}

#[derive(Insertable)]
#[diesel(table_name = consumers)]
pub struct NewConsumer<'a> {
    pub alias_id: i32,
    pub alias_name: &'a str,
    pub alias_pfp: &'a str,
    pub created_at: i32,
}

#[derive(Queryable)]
pub struct Savegame {
    pub consumer_id: i32,
    pub points: i32,
}

#[derive(Insertable)]
#[diesel(table_name = savegames)]
pub struct NewSavegame {
    pub consumer_id: i32,
}

#[derive(Queryable)]
pub struct Activity {
    pub id: i32,
    pub consumer_id: i32,
    pub action_id: i32,
    pub timestamp: i32,
}

#[derive(Insertable)]
#[diesel(table_name = activities)]
pub struct NewActivity {
    pub consumer_id: i32,
    pub action_id: i32,
    pub timestamp: i32,
}