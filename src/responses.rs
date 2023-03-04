use std::io::{self, Read};

use rocket::{
    data::{FromData, Transform},
    http::Status,
};
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

pub enum SipRequestError {
    Io(io::Error),
    Parse,
}

impl<'a> FromData<'a> for SipRequest {
    type Error = SipRequestError;
    type Owned = String;
    type Borrowed = str;

    fn transform(
        _request: &rocket::Request,
        data: rocket::Data,
    ) -> rocket::data::Transform<rocket::data::Outcome<Self::Owned, Self::Error>> {
        let mut stream = data.open();
        let mut string = String::new();

        let outcome = match stream.read_to_string(&mut string) {
            Ok(_) => rocket::Outcome::Success(string),
            Err(e) => {
                rocket::Outcome::Failure((Status::InternalServerError, SipRequestError::Io(e)))
            }
        };

        Transform::Borrowed(outcome)
    }

    fn from_data(
        _request: &rocket::Request,
        outcome: rocket::data::Transformed<'a, Self>,
    ) -> rocket::data::Outcome<Self, Self::Error> {
        let string: &str = outcome.borrowed()?;

        let result: Option<SipRequest> = serde_json::from_str(string).unwrap_or_else(|_| None);

        if result.is_none() {
            return rocket::Outcome::Failure((Status::UnprocessableEntity, SipRequestError::Parse));
        }

        rocket::Outcome::Success(result.unwrap())
    }
}
