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

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct IpAddressRegion {
    pub name: String,
    pub networkTier: String,
    ///Not supported in regional version
    //pub ipVersion: String,
    pub addressType: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct GetIpAddress {
  pub kind: String,
  pub id: String,
  pub creationTimestamp: String,
  pub name: String,
  pub description: String,
  pub address: String,
  pub status: String,
  pub selfLink: String,
  pub networkTier: String,
  pub labelFingerprint: String,
  pub ipVersion: String,
  pub addressType: String
}
