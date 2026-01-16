use serde_derive::{Serialize, Deserialize};

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct IPName {
    pub name: String,
}

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct IpAddress {
    pub name: String,
    pub address: String,
    pub prefixLength: String,
    pub networkTier: String,
    pub ipVersion: String,
    pub addressType: String,
    pub purpose: String,
    pub network: String,
}
