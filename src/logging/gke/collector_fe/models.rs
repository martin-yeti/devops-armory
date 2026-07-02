use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct InputForm {
    pub google_project_id: String,
    #[serde(default)]
    pub project_id: String,
    #[serde(default)]
    pub region: String,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub date_from: String,
    #[serde(default)]
    pub date_to: String,
}

#[derive(Deserialize, Serialize)]
pub struct Log {
    pub id: i32,
    pub google_project_id: String,
    pub project_id: String,
    pub region: String,
    pub host: String,
    pub message: String,
    pub time: String,
}
