use diesel::{insert_into, update, ExpressionMethods, QueryDsl, RunQueryDsl};
use rand::Rng;
use rocket::{http::Status, response::status::Custom, serde::json::Json};
use rocket_dyn_templates::{context, Template};
use std::{env, fs::File};
use twitch_api::{
    twitch_oauth2::{AccessToken, UserToken},
    TwitchClient,
};

use crate::{
    establish_connection,
    models::{NewUser, User},
    responses::{GenericResponse, SipProcessed, SipRequest},
    structs::Lines,
};

#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! {})
}

#[post("/sip", data = "<data>")]
pub async fn take_a_sip_of_tea(data: &str) -> Custom<Json<GenericResponse<SipProcessed>>> {
    let sip_request: SipRequest = serde_json::from_str(data).unwrap();

    dotenvy::dotenv().ok();

    use crate::schema::users::dsl::*;
    let client: TwitchClient<reqwest::Client> = TwitchClient::default();
    let token = match UserToken::from_existing(
        &reqwest::Client::new(),
        AccessToken::new(
            env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN must be set for Twitch API requests!"),
        ),
        None,
        None,
    )
    .await
    {
        Ok(t) => t,
        Err(e) => panic!("Couldn't get the token: {}", e),
    };

    let __id = sip_request.alias_id.to_string();
    let _id = __id.as_str();
    let ids: &[&twitch_api::types::UserIdRef] = &[_id.into()];
    let request = twitch_api::helix::users::GetUsersRequest::ids(ids);
    let res = &client.helix.req_get(request, &token).await.unwrap();
    let user = res.data.first().unwrap();

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
                alias_name: user.login.as_str(),
                alias_pfp: user
                    .profile_image_url
                    .clone()
                    .unwrap_or("".to_string())
                    .as_str(),
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

    let min_per_sip = env::var("MIN_PER_SIP")
        .unwrap_or("-10".to_string())
        .parse::<i32>()
        .unwrap();

    let max_per_sip = env::var("MAX_PER_SIP")
        .unwrap_or("50".to_string())
        .parse::<i32>()
        .unwrap();

    let mut _points = rand::thread_rng().gen_range(min_per_sip..max_per_sip);

    let file = File::open("./lines.json").unwrap();
    let lines: Lines = serde_json::from_reader(file).unwrap();

    let mut percent = (_points as f64 / max_per_sip as f64) * 100.0;
    let event_id = rand::thread_rng().gen_range(0..2);

    let message = match event_id {
        1 => {
            let mut consumers = users
                .filter(platform.eq(sip_request.platform))
                .filter(alias_id.ne(sip_request.alias_id.to_string()))
                .load::<User>(conn)
                .unwrap_or(Vec::new());

            let index = rand::thread_rng().gen_range(0..consumers.len());

            let mut consumer: &mut User = consumers.get_mut(index).unwrap();

            _points = rand::thread_rng().gen_range(1..max_per_sip);
            consumer.points -= _points;

            update(users.find(consumer.id))
                .set(points.eq(consumer.points))
                .execute(conn)
                .expect("Cannot update the values!");

            let __id = &consumer.alias_id;
            let _id = __id.as_str();
            let ids: &[&twitch_api::types::UserIdRef] = &[_id.into()];
            let request = twitch_api::helix::users::GetUsersRequest::ids(ids);
            let res = &client.helix.req_get(request, &token).await.unwrap();
            let user = res.data.first().unwrap();

            percent = (_points as f64 / max_per_sip as f64) * 100.0;

            format!(
                "you didn't get your portion of milk, but you {} took {} and pumped all the milk out of him 🥛 🥴 ",
                if percent >= 60.0 {
                    "rudely"
                } else if (25.0..60.0).contains(&percent) {
                    "gently"
                } else {
                    ""
                },
                user.login
            )
        }
        _ => if percent >= 90.0 {
            lines
                .legendary_lines
                .get(rand::thread_rng().gen_range(0..lines.legendary_lines.len()))
                .unwrap()
        } else if (50.0..90.0).contains(&percent) {
            lines
                .epic_lines
                .get(rand::thread_rng().gen_range(0..lines.epic_lines.len()))
                .unwrap()
        } else if (10.0..50.0).contains(&percent) {
            lines
                .common_lines
                .get(rand::thread_rng().gen_range(0..lines.common_lines.len()))
                .unwrap()
        } else {
            lines
                .poor_lines
                .get(rand::thread_rng().gen_range(0..lines.poor_lines.len()))
                .unwrap()
        }
        .to_string(),
    };

    _rs.points += _points;

    update(users.find(_rs.id))
        .set((
            points.eq(_rs.points),
            last_timestamp.eq(i32::try_from(chrono::Utc::now().timestamp()).unwrap()),
            alias_name.eq(user.login.to_string()),
            alias_pfp.eq(user.profile_image_url.clone().unwrap_or("".to_string())),
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
                message,
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

#[get("/leaderboard")]
pub fn get_leaderboard(
) -> Custom<Json<GenericResponse<Vec<(String, Option<String>, Option<String>, i32)>>>> {
    let conn = &mut establish_connection();
    let user = crate::schema::users::dsl::users
        .select((
            crate::schema::users::dsl::alias_id,
            crate::schema::users::dsl::alias_pfp,
            crate::schema::users::dsl::alias_name,
            crate::schema::users::dsl::points,
        ))
        .order(crate::schema::users::dsl::points.desc())
        .limit(10)
        .load::<(String, Option<String>, Option<String>, i32)>(conn);

    if user.is_err() {
        return Custom(
            Status::InternalServerError,
            Json(GenericResponse {
                status: 500,
                data: Vec::new(),
            }),
        );
    }

    Custom(
        Status::Ok,
        Json(GenericResponse {
            status: 200,
            data: user.unwrap(),
        }),
    )
}
