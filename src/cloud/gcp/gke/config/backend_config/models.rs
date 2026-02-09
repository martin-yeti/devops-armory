use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct BackendConfig {
    pub apiVersion: String,
    pub kind: String,
    pub metadata: BackendConfigMetadata,
    pub spec: BackendConfigSpec
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct BackendConfigMetadata {
    pub name: String,
    pub namespace: String
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct BackendConfigSpec {
    pub healthCheck: BackendConfigHealthcheck,
    pub sessionAffinity: BackendConfigSessionAffinity,
    pub connectionDraining: BackendConfigConnDrain,
    pub timeoutSec: i32,
    pub cdn: Option<CdnSettings>
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct BackendConfigHealthcheck {
    pub timeoutSec: i32,
    pub r#type: String,
    pub requestPath: String,
    pub port: i32,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct BackendConfigSessionAffinity {
    pub affinityType: String
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct BackendConfigConnDrain {
    pub drainingTimeoutSec: i32
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CdnSettings {
    pub enable: bool,
    pub cachePolicy: CachePolicy,
    pub cacheMode: String,
    pub clientTtl: i32,
    pub defaultTtl: i32,
    pub maxTtl: i32,
    pub negativeCaching: bool,
    pub negativeCachingPolicy: Vec<NegativeCache>,
    pub serveWhileStale: i32
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CachePolicy {
    pub includeHost: bool,
    pub includeQueryString: bool
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct NegativeCache {
    pub code: i32,
    pub ttl: i32
}

