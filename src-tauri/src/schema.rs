// @generated automatically by Diesel CLI.

diesel::table! {
    cards (id) {
        id -> Integer,
        card_number -> Text,
        card_name -> Nullable<Text>,
        user_fullname -> Nullable<Text>,
        user_id -> Nullable<Text>,
        is_present -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    punches (id) {
        id -> Integer,
        card_id -> Integer,
        status -> Text,
        timestamp -> Timestamp,
    }
}

diesel::joinable!(punches -> cards (card_id));

diesel::allow_tables_to_appear_in_same_query!(cards, punches,);
