use std::env;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use infrastructure::{establish_connection, models::Consumer, schema::consumers::dsl as cs};
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
