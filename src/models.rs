use crate::schema::users;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub alias_id: String,
    pub platform: i32,
    pub points: i32,
    pub inventory: Option<String>,
    pub created_timestamp: i32,
    pub last_timestamp: i32,
}
