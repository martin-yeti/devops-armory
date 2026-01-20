use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct VpcSubnet {
    pub name: String,
    pub description: String,
    pub network: String,
    pub ipCidrRange: String,
    pub region: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct VpcSubnetUpdate {
    pub secondaryIpRanges: Vec<SecondaryIpRanges>,
    pub fingerprint: String
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SecondaryIpRanges {
    pub rangeName: String,
    pub ipCidrRange: String,
    //pub reservedInternalRange: String
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct VpcSubnetPrivateIpGoogleAccess {
    pub privateIpGoogleAccess: bool
}

