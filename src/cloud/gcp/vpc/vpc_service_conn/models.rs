use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct PrivateServiceConnection {
    //pub service: String,
    pub network: String,
    pub reservedPeeringRanges: Vec<String>,
    //pub vpcPeeringConfig: VpcPeeringConfig,
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct VpcPeeringConfig {
    pub vpcNetwork: String,
}

