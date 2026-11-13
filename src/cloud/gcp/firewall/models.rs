use serde::{Serialize, Deserialize};

#[derive(Debug,Deserialize, Serialize)]
pub struct FirewallRule {
    pub name: String,
    pub description: Option<String>,
    pub network: String,
    pub priority: String,
    pub sourceRanges: Vec<String>,
    pub destinationRanges: Vec<String>,
    pub sourceTags: Vec<String>,
    pub targetTags: Vec<String>,
    pub allowed: Vec<Rules>,
    pub denied: Vec<Rules>,
    pub direction: String,
    pub logConfig: LogConfig,
    pub disabled: bool
}

#[derive(Debug,Deserialize, Serialize)]
pub struct Rules {
    pub IPProtocol: String,
    pub ports: Vec<String>
}

#[derive(Debug,Deserialize, Serialize)]
pub struct LogConfig {
    pub enable: bool,
}
