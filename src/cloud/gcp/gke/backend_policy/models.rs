use serde::{Deserialize, Serialize};
 
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GcpBackendPolicy {
    pub api_version: String,
    pub kind: String,
    pub metadata: ObjectMeta,
    pub spec: GcpBackendPolicySpec,
}
 
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectMeta {
    pub name: String,
    pub namespace: String,
}
 
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GcpBackendPolicySpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<BackendPolicyConfig>,
    pub target_ref: TargetRef,
}
 
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackendPolicyConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_policy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<LoggingConfig>,
}
 
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggingConfig {
    pub enabled: bool,
    /// Per million: 1_000_000 == 100 %.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<u32>,
}
 
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TargetRef {
    /// Core group is the empty string — must be serialized, not skipped.
    pub group: String,
    pub kind: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGcpBackendPolicy {
    pub spec: GcpBackendPolicySpec,
}
