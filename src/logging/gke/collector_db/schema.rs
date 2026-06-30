// @generated automatically by Diesel CLI.

diesel::table! {
    logs (id) {
        id -> Int4,
        google_project_id -> Text,
        project_id -> Text,
        region -> Text,
        host -> Text,
        message -> Text,
        time -> Nullable<Timestamptz>,
    }
}
