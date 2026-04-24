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


// Get instance info models

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct GetVMInstanceInfo {
    pub kind: String,
    pub id: String,
    pub creationTimestamp: String,
    pub name: String,
    pub tags: VMTagsGetVM,
    pub machineType: String,
    pub status: String,
    pub zone: String,
    //pub canIpForward: bool,
    pub networkInterfaces: Vec<VMNetInterfacesGet>,
    pub disks: Vec<VMDiskGet>,
    pub metadata: VMMetadataGet,
    //pub selfLink: String,
    //pub scheduling: VMSchedulingGet,
    //pub cpuPlatform: String,
    //pub labelFingerprint: String,
    //pub startRestricted: bool,
    //pub deletionProtection: bool,
    //pub shieldedInstanceConfig: VMSchieldedInstanceConfigGet,
    //pub shieldedInstanceIntegrityPolicy: VMSchieldedInstancePolicyGet,
    //pub fingerprint: String,
    //pub lastStartTimestamp: String,
    //pub resourceStatus: VMResourceStatusGet
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct VMTagsGetVM {
    pub items: Vec<String>,
    pub fingerprint: String
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct VMNetInterfacesGet {
    pub kind: String,
    pub network: String,
    pub subnetwork: String,
    pub networkIP: String,
    pub name: String,
    pub fingerprint: String,
    pub stackType: String
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct VMDiskGet {
    pub kind: String,
    pub r#type: String,
    pub mode: String,
    pub source: String,
    pub deviceName: String,
    pub index: i32,
    pub boot: bool,
    pub autoDelete: bool,
    pub licenses: Vec<String>,
    pub interface: String,
    pub diskSizeGb: String,
    //pub shieldedInstanceInitialState: Vec<VMShieldedInitialState>,
    pub architecture: String
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct VMShieldedInitialState {
    pub dbxs: Vec<VMDBXS>
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct VMDBXS {
    pub content: String,
    pub fileType: String
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct VMMetadataGet {
    pub kind: String,
    pub fingerprint: String
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct VMSchedulingGet {
    pub onHostMaintenance: String,
    pub automaticRestart: bool,
    pub preemptible: bool,
    pub provisioningModel: String
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct VMSchieldedInstanceConfigGet {
    pub enableSecureBoot: bool,
    pub enableVtpm: bool,
    pub enableIntegrityMonitoring: bool
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct VMSchieldedInstancePolicyGet {
    pub updateAutoLearnPolicy: bool
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct VMResourceStatusGet {
    pub effectiveInstanceMetadata: EffectiveInstanceMetadata
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct EffectiveInstanceMetadata {
    pub vmDnsSettingMetadataValue: String
}
