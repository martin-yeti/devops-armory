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
    pub message: String,
    pub date_from: chrono::DateTime<chrono::Utc>,
    pub date_to: chrono::DateTime<chrono::Utc>,
    pub offset: i64,
}

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
        logs::table
            .filter(logs::google_project_id.eq(google_project_id))
            .filter(logs::project_id.eq(project_id))
            .filter(logs::region.eq(region))
            .filter(logs::host.like(format!("%{}%", host)))
            .filter(logs::message.like(format!("%{}%", message)))
            .filter(logs::time.between(date_from, date_to))
            .order_by(logs::id.desc())
            .limit(10000)
            .offset(offset)
            .load(connection)
}

