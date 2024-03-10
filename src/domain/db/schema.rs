// @generated automatically by Diesel CLI.

diesel::table! {
    tweet (id) {
        id -> Uuid,
        body -> Text,
    }
}
