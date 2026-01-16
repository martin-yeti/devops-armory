use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct VpcNat {
    pub name: String,
    pub natIpAllocateOption: String,
    pub natIps: Vec<String>,
    //pub targetVpnGateways: Vec<_>,
    pub sourceSubnetworkIpRangesToNat: Vec<String>,
    pub minPortsPerVm: i32,
    pub logConfig: VpcNatLogConfig
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct VpcNatLogConfig {
    pub enable: bool,
    pub filter: String
}
