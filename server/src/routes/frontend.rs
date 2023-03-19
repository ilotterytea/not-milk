use std::env;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use infrastructure::{
    establish_connection,
    models::{Consumer, PointsHistory, Savegame},
    schema::{consumers::dsl as cs, points_history::dsl as ph, savegames::dsl as sg},
};
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
            bot_icon: env::var("BOT_ICON_URL").expect("BOT_ICON_URL must be set for frontend!")
        },
    )
}

#[get("/search?<query>")]
pub fn search(query: &str) -> Template {
    let conn = &mut establish_connection();

    let consumers = cs::consumers
        .filter(cs::alias_name.eq(query))
        .select((cs::alias_name, cs::alias_pfp))
        .load::<(String, String)>(conn)
        .expect("Couldn't load the consumers!");

    Template::render(
        "search",
        context! {
            length: consumers.len(),
            query: query,
            users: consumers,
        },
    )
}

#[get("/user/<id>")]
pub fn lookup_user(id: &str) -> Custom<Template> {
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
                    name: id
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

    Custom(
        rocket::http::Status::Ok,
        Template::render(
            "user",
            context! {
                name: consumer.alias_name,
                pfp: consumer.alias_pfp,
                current: savegame.points,
                activities: recent_activity
            },
        ),
    )
}

#[get("/leaderboard")]
pub fn leaderboard() -> Template {
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
            users: consumer_data
        },
    )
}
