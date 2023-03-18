// @generated automatically by Diesel CLI.

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
    points_history (id) {
        id -> Integer,
        consumer_id -> Integer,
        timestamp -> Integer,
        caused_by_consumer_id -> Nullable<Integer>,
        difference -> Integer,
        points_before_difference -> Integer,
    }
}

diesel::table! {
    savegames (consumer_id) {
        consumer_id -> Integer,
        points -> Integer,
    }
}

diesel::joinable!(lines -> channels (channel_id));

diesel::allow_tables_to_appear_in_same_query!(
    channels,
    consumers,
    lines,
    points_history,
    savegames,
);
