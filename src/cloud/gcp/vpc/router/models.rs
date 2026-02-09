use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct VpcRouter {
    pub name: String,
    pub nats: Vec<RouterNats>,
    pub network: String,
    pub region: String
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct RouterNats {
    pub natIpAllocateOption: String,
    pub natIps: Vec<String>,
    pub name: String,
    pub udpIdleTimeoutSec: i32,
    pub subnetworks: Vec<RouterSubnetConfig>,
    pub r#type: String,
    pub tcpTimeWaitTimeoutSec: i32,
    pub icmpIdleTimeoutSec: i32,
    pub tcpTransitoryIdleTimeoutSec: i32,
    pub endpointTypes: Vec<String>,
    pub tcpEstablishedIdleTimeoutSec: i32,
    pub enableEndpointIndependentMapping: bool,
    pub autoNetworkTier: String,
    pub sourceSubnetworkIpRangesToNat: String,
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct RouterSubnetConfig {
    pub sourceIpRangesToNat: Vec<String>,
    pub name: String,
}

