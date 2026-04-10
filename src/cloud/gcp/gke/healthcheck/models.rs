use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheckPolicy {
    pub apiVersion: String,
    pub kind: String,
    pub metadata: Metadata,
    pub spec: HealthCheckPolicySpec,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub namespace: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheckPolicySpec {
    pub default: DefaultHealthCheckConfig,
    pub targetRef: TargetRef,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultHealthCheckConfig {
    pub checkIntervalSec: u32,
    pub timeoutSec: u32,
    pub healthyThreshold: u32,
    pub unhealthyThreshold: u32,
    pub logConfig: LogConfig,
    pub config: HealthCheckConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogConfig {
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    pub r#type: String, 
    //pub httpHealthCheck: HttpHealthCheck,
    pub tcpHealthCheck: HttpHealthCheck
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpHealthCheck {
    pub port: u16,
    pub requestPath: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetRef {
    pub group: String,
    pub kind: String,
    pub name: String,
}

