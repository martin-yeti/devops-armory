use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheckPolicy {
    apiVersion: String,
    kind: String,
    metadata: Metadata,
    spec: HealthCheckPolicySpec,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    name: String,
    namespace: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheckPolicySpec {
    default: DefaultHealthCheckConfig,
    targetRef: TargetRef,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultHealthCheckConfig {
    checkIntervalSec: u32,
    timeoutSec: u32,
    healthyThreshold: u32,
    unhealthyThreshold: u32,
    logConfig: LogConfig,
    config: HealthCheckConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogConfig {
    enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    r#type: String, 
    httpHealthCheck: HttpHealthCheck,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpHealthCheck {
    port: u16,
    requestPath: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetRef {
    group: String,
    kind: String,
    name: String,
}

