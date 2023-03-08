// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        alias_id -> Text,
        platform -> Integer,
        points -> Integer,
        inventory -> Nullable<Text>,
        created_timestamp -> Integer,
        last_timestamp -> Integer,
        alias_name -> Nullable<Text>,
        alias_pfp -> Nullable<Text>,
    }
}
