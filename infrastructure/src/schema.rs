// @generated automatically by Diesel CLI.

diesel::table! {
    activities (id) {
        id -> Integer,
        consumer_id -> Integer,
        action_id -> Integer,
        timestamp -> Integer,
    }
}

diesel::table! {
    consumers (id) {
        id -> Integer,
        alias_id -> Integer,
        alias_name -> Text,
        alias_pfp -> Text,
        created_at -> Integer,
    }
}

diesel::table! {
    savegames (consumer_id) {
        consumer_id -> Integer,
        points -> Integer,
    }
}

diesel::joinable!(activities -> consumers (consumer_id));

diesel::allow_tables_to_appear_in_same_query!(
    activities,
    consumers,
    savegames,
);
