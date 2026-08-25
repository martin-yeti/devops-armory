use super::schema::pod_metrics;
use diesel::prelude::*;
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, QueryableByName, Apiv2Schema)]
#[diesel(table_name = super::schema::pod_metrics)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PodMetric {
    pub id: i32,
    pub google_project_id: String,
    pub project_id: String,
    pub region: String,
    pub namespace: String,
    pub pod_name: String,
    pub cpu_request: f64,
    pub ram_request: f64,
    pub cpu_limit: f64,
    pub ram_limit: f64,
    pub healthy: bool,
    pub cpu_usage: f64,
    pub ram_usage: f64,
    pub time: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = pod_metrics)]
pub struct NewPodMetric {
    pub google_project_id: String,
    pub project_id: String,
    pub region: String,
    pub namespace: String,
    pub pod_name: String,
    pub cpu_request: f64,
    pub ram_request: f64,
    pub cpu_limit: f64,
    pub ram_limit: f64,
    pub healthy: bool,
    pub cpu_usage: f64,
    pub ram_usage: f64,
    pub time: Option<chrono::DateTime<chrono::Utc>>,
}
