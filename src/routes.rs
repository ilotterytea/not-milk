use diesel::{insert_into, update, ExpressionMethods, QueryDsl, RunQueryDsl};
use rand::Rng;
use rocket::{http::Status, response::status::Custom};
use rocket_contrib::json::Json;
use std::env;

use crate::{
    establish_connection,
    models::{NewUser, User},
    responses::{GenericResponse, SipProcessed, SipRequest},
};

#[post("/sip", data = "<sip_request>")]
pub fn take_a_sip_of_tea(sip_request: SipRequest) -> Custom<Json<GenericResponse<SipProcessed>>> {
    dotenvy::dotenv().ok();

    use crate::schema::users::dsl::*;

    let conn = &mut establish_connection();
    let mut result = users
        .filter(alias_id.eq(sip_request.alias_id.to_string()))
        .filter(platform.eq(sip_request.platform))
        .first::<User>(conn);

    let is_new_user = result.is_err();
    let interval_time = env::var("INTERVAL_TIME_MS")
        .unwrap_or("10000".to_string())
        .parse::<i32>()
        .unwrap();

    if !is_new_user
        && i32::try_from(chrono::Utc::now().timestamp()).unwrap()
            - result.as_ref().unwrap().last_timestamp
            < interval_time
    {
        return Custom(
            Status::Forbidden,
            Json(GenericResponse {
                status: 403,
                data: SipProcessed {
                    delay: interval_time
                        - (i32::try_from(chrono::Utc::now().timestamp()).unwrap()
                            - result.as_ref().unwrap().last_timestamp),
                    message: "You are delayed now!".to_string(),
                    income: 0,
                    user: result.unwrap(),
                },
            }),
        );
    }

    if is_new_user {
        insert_into(users)
            .values(vec![NewUser {
                alias_id: sip_request.alias_id.to_string().as_str(),
                platform: sip_request.platform,
                created_timestamp: i32::try_from(chrono::Utc::now().timestamp()).unwrap(),
                last_timestamp: i32::try_from(chrono::Utc::now().timestamp()).unwrap(),
            }])
            .execute(conn)
            .expect("Cannot insert the values!");

        result = users
            .filter(alias_id.eq(sip_request.alias_id.to_string()))
            .filter(platform.eq(sip_request.platform))
            .first::<User>(conn);
    }

    let mut _rs = result.unwrap();

    let calories_in_gram = env::var("CALORIES_IN_GRAM")
        .unwrap_or("7".to_string())
        .parse::<i32>()
        .unwrap();

    let min_calories_per_sip = env::var("MIN_CALORIES_PER_SIP")
        .unwrap_or("175".to_string())
        .parse::<i32>()
        .unwrap();

    let max_calories_per_sip = env::var("MAX_CALORIES_PER_SIP")
        .unwrap_or("1260".to_string())
        .parse::<i32>()
        .unwrap();

    let _points =
        rand::thread_rng().gen_range(min_calories_per_sip..max_calories_per_sip) / calories_in_gram;

    _rs.points = _rs.points + _points;

    update(
        users
            .filter(alias_id.eq(sip_request.alias_id.to_string()))
            .filter(platform.eq(sip_request.platform)),
    )
    .set((
        points.eq(_rs.points),
        last_timestamp.eq(i32::try_from(chrono::Utc::now().timestamp()).unwrap()),
    ))
    .execute(conn)
    .expect("Cannot update the values!");

    Custom(
        Status::Ok,
        Json(GenericResponse {
            status: 200,
            data: SipProcessed {
                delay: interval_time
                    - (i32::try_from(chrono::Utc::now().timestamp()).unwrap() - _rs.last_timestamp),
                income: _points,
                message: "We'll see... We'll see... And the winner gets a tea... ".to_string(),
                user: _rs,
            },
        }),
    )
}

#[get("/get_user?<id>")]
pub fn get_user(id: String) -> Custom<Json<GenericResponse<Option<User>>>> {
    let conn = &mut establish_connection();
    let user = crate::schema::users::dsl::users
        .filter(crate::schema::users::dsl::alias_id.eq(id))
        .first::<User>(conn);

    if user.is_err() {
        return Custom(
            Status::NotFound,
            Json(GenericResponse {
                status: 404,
                data: None,
            }),
        );
    }

    Custom(
        Status::Ok,
        Json(GenericResponse {
            status: 200,
            data: Some(user.unwrap()),
        }),
    )
}
