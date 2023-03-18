use std::env;

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
