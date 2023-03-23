use std::{env, fs::create_dir_all};

use diesel::{insert_into, update, ExpressionMethods, QueryDsl, RunQueryDsl};
use image::{GenericImageView, ImageBuffer, Rgba, RgbaImage};
use infrastructure::{
    establish_connection,
    models::{Consumer, NewAction, NewNonFungibleMilk, NonFungibleMilk, Savegame},
    schema::{actions::dsl as act, non_fungible_milks::dsl as nfm, savegames::dsl as sg},
};
use rand::Rng;
use sha2::{Digest, Sha256};

use crate::utils::{humanize_timestamp_like_timer, ParsedMessage};

pub async fn run(consumer: Consumer, msg_args: &ParsedMessage) -> Option<String> {
    let conn = &mut establish_connection();

    let _latest_action_timestamp = act::actions
        .filter(act::consumer_id.eq(consumer.id))
        .filter(act::name.eq("nfm"))
        .select(act::created_at)
        .order(act::created_at.desc())
        .first::<i32>(conn);

    if _latest_action_timestamp.is_ok() {
        let latest_action_timestamp = _latest_action_timestamp.unwrap();
        let difference =
            i32::try_from(chrono::Utc::now().timestamp()).unwrap() - latest_action_timestamp;
        let delay = std::env::var("NFM_DELAY_SEC")
            .unwrap_or("300".to_string())
            .parse::<i32>()
            .unwrap();

        if difference < delay {
            return Some(format!(
                "{}: i am sorry, Master... but you have to wait {} for me to do NFM for u 🥛 ✋ ",
                consumer.alias_name,
                humanize_timestamp_like_timer(delay - difference)
            ));
        }
    }
    let mut savegame = sg::savegames
        .find(consumer.id)
        .first::<Savegame>(conn)
        .expect("Couldn't load the savegame!");

    let cost = env::var("NFM_COST")
        .unwrap_or("800".to_string())
        .parse::<i32>()
        .unwrap();

    if savegame.points < cost {
        return Some(format!(
            "{}: you need {} more 🥛 to forc-ce m-me to make you a new nfm ... 😏 ",
            consumer.alias_name,
            cost - savegame.points
        ));
    }

    let img = image::open("milk/01.png").unwrap();
    let (width, height) = img.dimensions();
    let mut out: RgbaImage = ImageBuffer::new(width, height);

    let mut bg: (i16, i16, i16);
    let mut fg: (i16, i16, i16);
    let mut fg2: (i16, i16, i16);
    let mut outline: (i16, i16, i16);
    let mut hash: String;

    loop {
        bg = (
            rand::thread_rng().gen_range(0..255),
            rand::thread_rng().gen_range(0..255),
            rand::thread_rng().gen_range(0..255),
        );

        fg = (
            rand::thread_rng().gen_range(0..255),
            rand::thread_rng().gen_range(0..255),
            rand::thread_rng().gen_range(0..255),
        );

        outline = (
            if fg.0 - 100 < 0 { 0 } else { fg.0 - 100 },
            if fg.1 - 100 < 0 { 0 } else { fg.1 - 100 },
            if fg.2 - 100 < 0 { 0 } else { fg.2 - 100 },
        );

        fg2 = (
            if fg.0 - 25 < 0 { 0 } else { fg.0 - 25 },
            if fg.1 - 25 < 0 { 0 } else { fg.1 - 25 },
            if fg.2 - 25 < 0 { 0 } else { fg.2 - 25 },
        );

        let hash_raw = format!(
            "{}:{},{},{},{}:{},{},{},{}:{},{},{},{}:{},{},{},{}",
            "01",
            &bg.0,
            &bg.1,
            &bg.2,
            255,
            &fg.0,
            &fg.1,
            &fg.2,
            255,
            &outline.0,
            &outline.1,
            &outline.2,
            255,
            &fg2.0,
            &fg2.1,
            &fg2.2,
            255
        );

        let mut hasher = Sha256::new();

        hasher.update(hash_raw);

        hash = format!("{:X}", hasher.finalize()).to_lowercase();

        let _milk = nfm::non_fungible_milks
            .filter(nfm::hash_sum.eq(&hash))
            .first::<NonFungibleMilk>(conn);

        if _milk.is_err() {
            break;
        }
    }

    let (replace_bg, replace_outline, replace_fg1, replace_fg2) = (
        [233, 185, 255, 255],
        [36, 25, 30, 255],
        [255, 178, 218, 255],
        [255, 130, 196, 255],
    );

    for mut pixel in img.pixels() {
        if pixel.2.eq(&Rgba(replace_bg)) {
            pixel.2 = Rgba([bg.0 as u8, bg.1 as u8, bg.2 as u8, 255]);
        } else if pixel.2.eq(&Rgba(replace_outline)) {
            pixel.2 = Rgba([outline.0 as u8, outline.1 as u8, outline.2 as u8, 255]);
        } else if pixel.2.eq(&Rgba(replace_fg1)) {
            pixel.2 = Rgba([fg.0 as u8, fg.1 as u8, fg.2 as u8, 255]);
        } else if pixel.2.eq(&Rgba(replace_fg2)) {
            pixel.2 = Rgba([fg2.0 as u8, fg2.1 as u8, fg2.2 as u8, 255]);
        }

        out.put_pixel(pixel.0, pixel.1, pixel.2);
    }

    insert_into(act::actions)
        .values(vec![NewAction {
            consumer_id: consumer.id,
            name: "nfm",
            body: if msg_args.message.is_some() {
                Some(msg_args.message.as_ref().unwrap().as_str())
            } else {
                None
            },
            raw: msg_args.raw_message.as_str(),
            created_at: i32::try_from(chrono::Utc::now().timestamp()).unwrap(),
        }])
        .execute(conn)
        .expect("Couldn't insert a new action!");

    insert_into(nfm::non_fungible_milks)
        .values(vec![NewNonFungibleMilk {
            consumer_id: consumer.id,
            hash_sum: hash.as_str(),
            created_at: i32::try_from(chrono::Utc::now().timestamp()).unwrap(),
        }])
        .execute(conn)
        .expect("Couldn't create a new NFM!");

    savegame.points -= cost;

    update(sg::savegames.find(consumer.id))
        .set(sg::points.eq(savegame.points))
        .execute(conn)
        .expect("Couldn't update the savegame!");

    create_dir_all("static/nfms").unwrap();
    out.save(format!("static/nfms/{}.png", &hash)).unwrap();

    Some(format!("{}: here's your UNIQUE generated NFM 🦇 🌰  👉  {}/static/nfms/{}.png ... keep it a secret, cuz someone will want to take a screenshot of it 🤫 ", consumer.alias_name, env::var("BASE_URL").expect("BASE_URL must be set for NFM command!"), hash))
}
