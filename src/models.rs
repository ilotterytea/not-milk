use crate::schema::users;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Serialize, Clone)]
pub struct User {
    pub id: i32,
    pub alias_id: String,
    pub platform: i32,
    pub points: i32,
    pub inventory: Option<String>,
    pub created_timestamp: i32,
    pub last_timestamp: i32,
    pub alias_name: Option<String>,
    pub alias_pfp: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub alias_id: &'a str,
    pub alias_name: &'a str,
    pub alias_pfp: &'a str,
    pub platform: i32,
    pub created_timestamp: i32,
    pub last_timestamp: i32,
}
