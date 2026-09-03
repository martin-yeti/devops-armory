// @generated automatically by Diesel CLI.

diesel::table! {
    pod_metrics (id) {
        id -> Int4,
        google_project_id -> Text,
        project_id -> Text,
        region -> Text,
        namespace -> Text,
        pod_name -> Text,
        cpu_request -> Float8,
        ram_request -> Float8,
        cpu_limit -> Float8,
        ram_limit -> Float8,
        healthy -> Bool,
        cpu_usage -> Float8,
        ram_usage -> Float8,
        time -> Nullable<Timestamptz>,
        reason -> Nullable<Text>,
    }
}
