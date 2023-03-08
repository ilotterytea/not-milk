use serde::{Deserialize, Serialize};

use crate::models::User;

#[derive(Serialize)]
pub struct GenericResponse<T> {
    pub status: u32,
    pub data: T,
}

#[derive(Deserialize)]
pub struct SipRequest {
    pub alias_id: i32,
    pub platform: i32,
}

#[derive(Serialize)]
pub struct SipProcessed {
    pub delay: i32,
    pub message: String,
    pub income: i32,
    pub user: User,
}
