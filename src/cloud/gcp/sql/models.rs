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

impl SqlIpConfig {
    pub fn new(
        project: String,
        net_name: String,
        ssl_mode: String,
        ipv4enabled: bool,
        require_ssl: bool
    ) -> Self {
            Self {
                privateNetwork: format!("projects/{project}/global/networks/{net_name}"),
                sslMode: ssl_mode,
                ipv4Enabled: ipv4enabled,
                requireSsl: require_ssl
        }
    }
}
