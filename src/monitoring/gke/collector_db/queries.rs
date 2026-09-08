use super::models::PodMetric;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use paperclip::actix::Apiv2Schema;
use super::schema::pod_metrics;


#[derive(Apiv2Schema, Deserialize, Serialize)]
pub struct ListPodMetricsQuery {
    pub project_id: String,
    pub region: String,
    pub namespace: String,
    #[serde(default)]
    pub pod_name: String,
    pub date_from: chrono::DateTime<chrono::Utc>,
    pub date_to: chrono::DateTime<chrono::Utc>,
    pub offset: i64,
}

/// Query to be executed when passing parameters
pub fn get_gcp_pod_metrics_by_project_id (
    google_project_id: String,
    project_id: &str,
    region: &str,
    namespace: &str,
    pod_name: &str,
    date_from: chrono::DateTime<chrono::Utc>,
    date_to: chrono::DateTime<chrono::Utc>,
    offset: i64,
    connection: &mut PgConnection,
) -> QueryResult<Vec<PodMetric>> {
        pod_metrics::table
            .filter(pod_metrics::google_project_id.eq(google_project_id))
            .filter(pod_metrics::project_id.eq(project_id))
            .filter(pod_metrics::region.eq(region))
            .filter(pod_metrics::namespace.eq(namespace))
            .filter(pod_metrics::pod_name.like(format!("%{}%", pod_name)))
            .filter(pod_metrics::time.between(date_from, date_to))
            .order_by(pod_metrics::id.desc())
            .limit(2000)
            .offset(offset)
            .load(connection)
}
