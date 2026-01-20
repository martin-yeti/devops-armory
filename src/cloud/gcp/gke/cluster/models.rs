use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct CreateGkeCluster {
    pub cluster: GkeCluster
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct GkeCluster {
    pub name: String,
    pub description: String,
    pub loggingService: Option<String>,
    pub monitoringService: Option<String>,
    pub network: String,
    //pub clusterIpv4Cidr: String,
    pub subnetwork: String,
    pub nodePools: Vec<Nodepools>,
    pub locations: Vec<String>,
    //pub resourceLabels: HashMap<String, String>,
    //pub masterAuthorizedNetworksConfig: MasterAuthorizedNetworksConfig,
    pub networkConfig: NetworkConfig,
    pub autoscaling: AutoscalingBase,
    pub ipAllocationPolicy: IpAllocationPolicy

}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct Nodepools {
    pub name: String,
    pub config: NodeConfig,
    pub initialNodeCount: i32,

}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct NodeConfig {
    pub machineType: String,
    pub diskSizeGb: i32,
    pub oauthScopes: Vec<String>,
    //pub metadata: GkeMetadata HashMap<String, String>,
    pub imageType: String,
    pub serviceAccount: String,
    //pub labels: HashMap<String, String>,
    pub diskType: String,
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct NetworkConfig {
    //pub podIpv4CidrBlock: String,
    pub defaultEnablePrivateNodes: bool,
    pub subnetwork: String,
    //pub networkTierConfig: NetworkTierConfig
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct IpAllocationPolicy {
    pub useIpAliases: bool,
    pub servicesIpv4CidrBlock: String,
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct NetworkTierConfig {
    pub networkTier: String
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct MasterAuthorizedNetworksConfig {
    pub enabled: bool,
    pub cidrBlocks: Vec<CiDrBlock>,
    pub gcpPublicCidrsAccessEnabled: bool,
    pub privateEndpointEnforcementEnabled: bool
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct CiDrBlock {
    pub displayName: String,
    pub cidrBlock: String
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct Autoscaling {
    pub enableNodeAutoprovisioning: bool,
    pub resourceLimits: Vec<AutoscalerResourceLimits>,
    pub autoscalingProfile: String,
    pub defaultComputeClassConfig: DefaultComputeClass
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct DefaultComputeClass {
    pub enabled: bool
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct AutoscalerResourceLimits {
    pub resourceType: String,
    pub minimum: String,
    pub maximum: String
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct AutoscalingBase {
    pub enabled: bool,
    pub maxNodeCount: i32,
    pub locationPolicy: String
}

/// Update GKE cluster

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct UpdateGkeCluster {
    pub projectId: String,
    //pub zone: String,
    pub clusterId: String,
    pub update: UpdateCluster
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct UpdateCluster {
    pub desiredMasterAuthorizedNetworksConfig: Option<MasterAuthorizedNetworksConfig>,
    pub desiredLoggingService: Option<String>,
    pub desiredMonitoringService: Option<String>,
    pub desiredMonitoringConfig: Option<DesiredMonitoringConfig>,

}

/// Update GKE cluster - Logging and monitoring

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct UpdateClusterLogging {
    pub projectId: String,
    pub clusterId: String,
    pub loggingService: String
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct UpdateClusterMonitoring {
    pub projectId: String,
    pub clusterId: String,
    pub monitoringService: String
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct DesiredMonitoringConfig{
    pub componentConfig: MonitoringComponentConfig
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct MonitoringComponentConfig{
    pub enableComponents: Vec<String>
}
