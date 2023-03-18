use std::env;

use diesel::{insert_into, update, ExpressionMethods, QueryDsl, RunQueryDsl};
use infrastructure::{
    establish_connection,
    models::{Consumer, NewConsumer, NewSavegame},
    schema::{consumers::dsl as cs, savegames::dsl as sg},
};
use twitch_api::{
    helix::users::GetUsersRequest,
    twitch_oauth2::{AccessToken, UserToken},
    types::UserIdRef,
    TwitchClient,
};

pub async fn sync_consumer(user_id: &str) -> Option<Consumer> {
    let client: TwitchClient<reqwest::Client> = TwitchClient::default();

    let token = match UserToken::from_existing(
        &reqwest::Client::new(),
        AccessToken::new(
            env::var("TWITCH_ACCESS_TOKEN")
                .expect("TWITCH_ACCESS_TOKEN must be set for Twitch API requests!"),
        ),
        None,
        None,
    )
    .await
    {
        Ok(t) => t,
        Err(e) => panic!("Got error: {}", e),
    };

    let ids: &[&UserIdRef] = &[user_id.into()];

    let users = &client
        .helix
        .req_get(GetUsersRequest::ids(ids), &token)
        .await
        .expect("Couldn't send a request!");

    let user = users.data.first()?;

    let conn = &mut establish_connection();
    let mut consumer = cs::consumers
        .filter(cs::alias_id.eq(user_id.parse::<i32>().unwrap()))
        .first::<Consumer>(conn)
        .unwrap_or_else(|_| {
            insert_into(cs::consumers)
                .values(vec![NewConsumer {
                    alias_id: user_id.parse::<i32>().unwrap(),
                    alias_name: user.login.as_str(),
                    alias_pfp: user.profile_image_url.as_ref().unwrap().as_str(),
                    created_at: i32::try_from(chrono::Utc::now().timestamp()).unwrap(),
                }])
                .execute(conn)
                .expect("Couldn't create a new user!");

            let c = cs::consumers
                .filter(cs::alias_id.eq(user_id.parse::<i32>().unwrap()))
                .first::<Consumer>(conn)
                .expect("Couldn't get the user!");

            // Create a savegame for a new user:
            insert_into(sg::savegames)
                .values(vec![NewSavegame { consumer_id: c.id }])
                .execute(conn)
                .expect("Couldn't create a new savegame!");

            c
        });

    consumer.alias_name = user.login.to_string();
    consumer.alias_pfp = user.profile_image_url.as_ref().unwrap().to_owned();

    update(cs::consumers.find(consumer.id))
        .set((
            cs::alias_pfp.eq(&consumer.alias_pfp),
            cs::alias_name.eq(&consumer.alias_name),
        ))
        .execute(conn)
        .expect("Couldn't update the user's alias pfp and name!");

    Some(consumer)
}

pub fn humanize_timestamp_like_timer(timestamp: i32) -> String {
    let d = (timestamp as f64 / (60.0 * 60.0 * 24.0)) as i32;
    let h = (timestamp as f64 / (60.0 * 60.0) % 24.0) as i32;
    let m = (timestamp as f64 % (60.0 * 60.0) / 60.0) as i32;
    let s = (timestamp as f64 % 60.0) as i32;

    if d == 0 && h == 0 && m == 0 {
        format!("{}s", s)
    } else if d == 0 && h == 0 {
        format!("{}m{}s", m, s)
    } else if d == 0 {
        format!("{}h{}m", h, m)
    } else {
        format!("{}d{}h", d, h)
    }
}
