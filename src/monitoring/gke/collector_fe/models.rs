use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct InputForm {
    pub google_project_id: String,
    #[serde(default)]
    pub project_id: String,
    #[serde(default)]
    pub region: String,
    #[serde(default)]
    pub namespace: String,
    #[serde(default)]
    pub pod_name: String,
    #[serde(default)]
    pub date_from: String,
    #[serde(default)]
    pub date_to: String,
}

#[derive(Deserialize, Serialize)]
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
    pub reason: Option<String>,
}
