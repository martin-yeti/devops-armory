use serde::Deserialize;

#[derive(Deserialize, Default,Debug)]
pub struct NodeList {
    pub items: Vec<NodeItems>
}

#[derive(Deserialize, Default,Debug,Clone)]
pub struct NodeItems {
    pub metadata: NodeMetadata
}

#[derive(Deserialize, Default,Debug, Clone)]
pub struct NodeMetadata {
    pub name: String
}

#[derive(Deserialize, Default,Debug, Clone)]
pub struct NodePool {
    pub nodePools: Vec<NodePoolInfo>
}

#[derive(Deserialize, Default,Debug, Clone)]
pub struct NodePoolInfo {
    pub name: String,
    pub config: NodePoolConfig,
    pub etag: String
}

#[derive(Deserialize, Default,Debug, Clone)]
pub struct NodePoolConfig {
    pub machineType: String,
    pub diskSizeGb: i64,
    pub metadata: NodePoolConfigMetadata,
    pub imageType: String,
    pub serviceAccount: String,
    pub effectiveCgroupMode: String,
}

#[derive(Deserialize, Default,Debug, Clone)]
pub struct NodePoolConfigMetadata {
    #[serde(rename="disable-legacy-endpoints")]
    pub disable_legacy_endpoint: String
}
