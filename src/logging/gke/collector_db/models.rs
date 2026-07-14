use super::schema::logs;
use diesel::prelude::*;
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, QueryableByName, Apiv2Schema)]
#[diesel(table_name = super::schema::logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Log {
    pub id: i32,
    pub google_project_id: String,
    pub project_id: String,
    pub region: String,
    pub host: String,
    pub message: String,
    pub time: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = logs)]
pub struct NewLog {
    pub google_project_id: String,
    pub project_id: String,
    pub region: String,
    pub host: String,
    pub message: String,
    pub time: Option<chrono::DateTime<chrono::Utc>>,
}
