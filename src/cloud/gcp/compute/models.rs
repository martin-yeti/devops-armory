use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CreateVM {
    pub disk: Vec<VMDisk>,
    pub machineType: String,
    pub name: String,
    pub networkInterfaces: Vec<VMNetworkInterface>,
    pub tags: VMTags,
    //pub auth: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct VMDisk {
    pub boot: String,
    pub r#initializeParams: VMDiskInitParams,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct VMDiskInitParams {
    pub sourceImage: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct VMNetworkInterface {
    pub accessConfigs: Option<Vec<VMAccessConfig>>,
    pub network: String,
    pub subnetwork: String
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct VMAccessConfig {
    pub name: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct IPName {
    pub name: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CreateSSL {
    pub name: String,
    pub managed: DomainsManaged,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct DomainsManaged {
    pub domains: Vec<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct VMTags {
    pub items: Vec<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ComputeMetadata {
    pub items: Vec<ComputeData>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ComputeData {
    pub key: String,
    pub value: String
}
