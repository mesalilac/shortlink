// @generated automatically by Diesel CLI.

diesel::table! {
    urls (id) {
        id -> Text,
        long_url -> Text,
        clicks -> Int4,
        expires_at -> Nullable<Int8>,
        max_clicks -> Nullable<Int4>,
        disabled -> Bool,
        last_clicked_at -> Nullable<Int8>,
        created_at -> Int8,
    }
}
