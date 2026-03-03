use serde::*;
use serde::Deserialize as SerDel;
use serde_query::Deserialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Pod {
    #[query(".items.[].metadata.name")]
    pub hostname: Vec<String>,
}

#[derive(Serialize)]
pub struct Log {
    pub google_project_id: String,
    pub project_id: String,
    pub region: String,
    pub host: String,
    pub time: String,
    pub message: String,
}

#[derive(Serialize, Eq, Hash, PartialEq, Clone, Debug, SerDel)]
pub struct Notification {
    pub channel: String,
    pub username: String,
    pub text: String,
    pub icon_emoji: String,
}
