use regex::Regex;
use std::env;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use infrastructure::{
    establish_connection,
    models::{Consumer, PointsHistory, Savegame},
    schema::{
        consumers::dsl as cs, non_fungible_milks::dsl as nfm, points_history::dsl as ph,
        savegames::dsl as sg,
    },
};
use rand::Rng;
use rocket::{http::Status, response::status::Custom};
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn home() -> Template {
    dotenvy::dotenv().ok();

    Template::render(
        "index",
        context! {
            bot_name: env::var("BOT_NAME").expect("BOT_NAME must be set for frontend!"),
            bot_maintainer: env::var("BOT_MAINTAINER").expect("BOT_MAINTAINER must be set for frontend!"),
            bot_icon: env::var("BOT_ICON_URL").expect("BOT_ICON_URL must be set for frontend!"),
            rate: env::var("NFM_DELTA")
        .unwrap_or_else(|_| {
            let c = ((rand::thread_rng().gen_range(-1.0..100.0)) / 10000.0).to_string();
            env::set_var("NFM_DELTA", &c);
            c
        })
        .parse::<f32>()
        .unwrap(),
        },
    )
}

#[get("/search?<query>")]
pub fn search(query: &str) -> Template {
    dotenvy::dotenv().ok();
    let conn = &mut establish_connection();

    let all_consumers = cs::consumers
        .select((cs::alias_name, cs::alias_pfp))
        .load::<(String, String)>(conn)
        .expect("Couldn't load the consumers!");

    let mut consumers: Vec<&(String, String)> = vec![];
    let matcher = Regex::new(format!(r"(?i){}", query).as_str()).unwrap();

    for consumer in &all_consumers {
        if matcher.is_match(consumer.0.as_str()) {
            consumers.push(consumer);
        }
    }

    Template::render(
        "search",
        context! {
            length: consumers.len(),
            query: query,
            users: consumers,
            bot_name: env::var("BOT_NAME").expect("BOT_NAME must be set for frontend!"),
        },
    )
}

#[get("/user/<id>")]
pub fn lookup_user(id: &str) -> Custom<Template> {
    dotenvy::dotenv().ok();
    let conn = &mut establish_connection();

    let _consumer = cs::consumers
        .filter(cs::alias_name.eq(id))
        .first::<Consumer>(conn);

    if _consumer.is_err() {
        return Custom(
            Status::NotFound,
            Template::render(
                "404",
                context! {
                        name: id,
                        bot_name: env::var("BOT_NAME").expect("BOT_NAME must be set for frontend!"),
                },
            ),
        );
    }

    let consumer = _consumer.unwrap();
    let savegame = sg::savegames
        .find(consumer.id)
        .first::<Savegame>(conn)
        .expect("Couldn't load the savegame!");

    let activities = ph::points_history
        .filter(ph::consumer_id.eq(consumer.id))
        .load::<PointsHistory>(conn)
        .expect("Couldn't get the points history!");

    let mut recent_activity: Vec<(i32, String, i32, i32)> = vec![];

    for activity in &activities {
        let message = if activity.caused_by_consumer_id.is_some() {
            let c = cs::consumers
                .find(activity.caused_by_consumer_id.unwrap())
                .first::<Consumer>(conn)
                .expect("Couldn't get the consumer!");

            format!("{} pumped the milk out of you!", c.alias_name,)
        } else {
            "Earned via 🥛 sip command!".to_string()
        };

        recent_activity.push((
            activity.timestamp,
            message,
            activity.difference,
            activity.points_before_difference,
        ));
    }

    let nfms = nfm::non_fungible_milks
        .filter(nfm::consumer_id.eq(consumer.id))
        .filter(nfm::sold.ne(1))
        .select((nfm::hash_sum, nfm::id))
        .load::<(String, i32)>(conn)
        .expect("Couldn't load the NFMs");

    Custom(
        rocket::http::Status::Ok,
        Template::render(
            "user",
            context! {
                name: consumer.alias_name,
                pfp: consumer.alias_pfp,
                current: savegame.points,
                activities: recent_activity,
                nfms,
                bot_name: env::var("BOT_NAME").expect("BOT_NAME must be set for frontend!"),
            },
        ),
    )
}

#[get("/leaderboard")]
pub fn leaderboard() -> Template {
    dotenvy::dotenv().ok();
    let conn = &mut establish_connection();

    let savegames = sg::savegames
        .order(sg::points.desc())
        .limit(50)
        .load::<Savegame>(conn)
        .expect("Couldn't load the savegames!");

    let mut consumer_data: Vec<(String, String, i32)> = vec![];

    for savegame in &savegames {
        let c = cs::consumers
            .find(savegame.consumer_id)
            .first::<Consumer>(conn)
            .expect("Couldn't get the consumer!");

        consumer_data.push((c.alias_name, c.alias_pfp, savegame.points));
    }

    Template::render(
        "leaderboard",
        context! {
            length: savegames.len(),
            users: consumer_data,
            bot_name: env::var("BOT_NAME").expect("BOT_NAME must be set for frontend!"),
        },
    )
}
