use super::models::Log;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use paperclip::actix::Apiv2Schema;
use super::schema::logs;


#[derive(Apiv2Schema, Deserialize, Serialize)]
pub struct ListLogsQuery {
    pub project_id: String,
    pub region: String,
    pub host: String,
    #[serde(default)]
    pub message: String,
    pub date_from: chrono::DateTime<chrono::Utc>,
    pub date_to: chrono::DateTime<chrono::Utc>,
    pub offset: i64,
}

/// Query to be executed when passing parameters
pub fn get_gcp_logs_by_project_id (
    google_project_id: String,
    project_id: &str,
    region: &str,
    host: &str,
    message: &str,
    date_from: chrono::DateTime<chrono::Utc>,
    date_to: chrono::DateTime<chrono::Utc>,
    offset: i64,
    connection: &mut PgConnection,
) -> QueryResult<Vec<Log>> {
        let mut query = logs::table
            .filter(logs::google_project_id.eq(google_project_id))
            .filter(logs::project_id.eq(project_id))
            .filter(logs::region.eq(region))
            .filter(logs::time.between(date_from, date_to))
            .into_boxed();

        // A substring LIKE can't use an index and runs on every row in the
        // time range, so skip it when the caller didn't ask for it rather
        // than evaluating a match-everything LIKE '%%'.
        if !host.is_empty() {
            query = query.filter(logs::host.like(format!("%{}%", host)));
        }
        if !message.is_empty() {
            query = query.filter(logs::message.like(format!("%{}%", message)));
        }

        // Ordering by time lets Postgres walk logs_project_host_idx
        // backwards and stop after offset + limit rows; ordering by id
        // forces it to load and sort every match in the range first.
        query
            .order_by((logs::time.desc(), logs::id.desc()))
            .limit(2000)
            .offset(offset)
            .load(connection)
}

