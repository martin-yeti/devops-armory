use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct SqlInstance {
    pub name: String,
    pub databaseVersion: String,
    pub settings: SqlSettings,
    pub region: String,
    pub project: String,
    pub rootPassword: String,

}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct SqlSettings {
    pub tier: String,
    pub userLabels: SqlUserLabels,
    pub ipConfiguration: SqlIpConfig,
    pub dataDiskType: String,

}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct SqlUserLabels {
    pub created_by: String
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct SqlIpConfig {
    pub privateNetwork: String,
    pub sslMode: String,
    pub ipv4Enabled: bool,
    pub requireSsl: bool,

}
