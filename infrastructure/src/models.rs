use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Consumer {
    pub id: i32,
    pub alias_id: i32,
    pub alias_name: String,
    pub alias_pfp: String,
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
pub struct PointsHistory {
    pub id: i32,
    pub consumer_id: i32,
    pub timestamp: i32,
    pub caused_by_consumer_id: Option<i32>,
    pub difference: i32,
    pub points_before_difference: i32,
}

#[derive(Insertable)]
#[diesel(table_name = points_history)]
pub struct NewPointsHistory {
    pub consumer_id: i32,
    pub caused_by_consumer_id: Option<i32>,
    pub difference: i32,
    pub points_before_difference: i32,
    pub timestamp: i32,
}

#[derive(Queryable)]
pub struct Channel {
    pub id: i32,
    pub alias_id: i32,
    pub is_parted: i32,
    pub joined_at: i32,
}

#[derive(Insertable)]
#[diesel(table_name = channels)]
pub struct NewChannel {
    pub alias_id: i32,
    pub joined_at: i32,
}

#[derive(Queryable)]
pub struct Suspension {
    pub consumer_id: i32,
    pub reason: Option<String>,
    pub duration: i32,
    pub timestamp: i32,
}

#[derive(Insertable)]
#[diesel(table_name = suspensions)]
pub struct NewSuspension<'a> {
    pub consumer_id: i32,
    pub reason: Option<&'a str>,
    pub duration: i32,
    pub timestamp: i32,
}
