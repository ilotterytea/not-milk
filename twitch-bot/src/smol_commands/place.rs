use crate::utils::sync_consumer;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use infrastructure::{establish_connection, models::Savegame, schema::savegames::dsl as sg};

pub async fn run(user_id: &str) -> Option<String> {
    let consumer = sync_consumer(user_id).await?;

    let conn = &mut establish_connection();
    let savegames = sg::savegames
        .order(sg::points.desc())
        .load::<Savegame>(conn)
        .expect("Couldn't get the savegames!");

    let pos = savegames
        .iter()
        .position(|p| p.consumer_id.eq(&consumer.id))
        .unwrap();

    Some(format!(
        "{}: you're in #{} place out of {}! {} {}",
        consumer.alias_name,
        pos + 1,
        savegames.len(),
        match pos {
            0 => "🥇 ",
            1 => "🥈 ",
            2 => "🥉 ",
            _ => "🏆 ",
        },
        if pos.ne(&0) {
            let next_savegame = savegames.get(pos - 1).unwrap();
            let my_savegame = savegames.get(pos).unwrap();

            let difference = next_savegame.points - my_savegame.points;

            format!(
                "| you need {} more 🥛 to level up your place on the leaderboard! forsenLevel",
                difference + 1
            )
        } else {
            "".to_string()
        }
    ))
}
