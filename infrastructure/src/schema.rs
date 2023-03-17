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
    channels (id) {
        id -> Integer,
        alias_id -> Integer,
        is_parted -> Integer,
        joined_at -> Integer,
    }
}

diesel::table! {
    consumers (id) {
        id -> Integer,
        alias_id -> Integer,
        alias_name -> Text,
        alias_pfp -> Text,
        is_suspended -> Integer,
        created_at -> Integer,
    }
}

diesel::table! {
    lines (id) {
        id -> Integer,
        line -> Text,
        category_id -> Integer,
        channel_id -> Nullable<Integer>,
        is_disabled -> Integer,
    }
}

diesel::table! {
    savegames (consumer_id) {
        consumer_id -> Integer,
        points -> Integer,
    }
}

diesel::joinable!(activities -> consumers (consumer_id));
diesel::joinable!(lines -> channels (channel_id));

diesel::allow_tables_to_appear_in_same_query!(
    activities,
    channels,
    consumers,
    lines,
    savegames,
);
