use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct VpcNetwork {
    pub name: String,
    pub description: String,
    pub autoCreateSubnetworks: bool,
    //pub IPv4Range: String,
    //pub subnetworks: Vec<String>,
}

