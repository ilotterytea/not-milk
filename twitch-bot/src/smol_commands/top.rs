use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use infrastructure::{
    establish_connection,
    models::{Consumer, Savegame},
    schema::{consumers::dsl as cs, savegames::dsl as sg},
};

pub fn run() -> Option<String> {
    let conn = &mut establish_connection();

    let savegames = sg::savegames
        .order(sg::points.desc())
        .limit(5)
        .load::<Savegame>(conn)
        .expect("Couldn't get the savegames!");

    let mut strings: Vec<String> = vec![];
    let mut index = 0;

    for savegame in &savegames {
        let consumer = cs::consumers
            .find(savegame.consumer_id)
            .first::<Consumer>(conn)
            .expect("Couldn't get the consumer!");

        index += 1;

        strings.push(format!(
            "{} {}. {} ({})",
            match index {
                1 => "🥇 ",
                2 => "🥈 ",
                3 => "🥉 ",
                _ => "",
            },
            index,
            consumer.alias_name,
            savegame.points
        ));
    }

    Some(format!(
        "🥛 🏆 top {} milk sippers: {}",
        index,
        strings.join(", ")
    ))
}
