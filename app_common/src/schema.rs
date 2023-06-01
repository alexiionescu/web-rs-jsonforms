// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        user -> Text,
        name -> Text,
        password -> Binary,
        json_state -> Text,
    }
}
