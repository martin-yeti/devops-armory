use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheckPolicy {
    api_version: String,
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
    target_ref: TargetRef,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultHealthCheckConfig {
    check_interval_sec: u32,
    timeout_sec: u32,
    healthy_threshold: u32,
    unhealthy_threshold: u32,
    log_config: LogConfig,
    config: HealthCheckConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogConfig {
    enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    r#type: String, 
    http_health_check: HttpHealthCheck,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpHealthCheck {
    port: u16,
    request_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetRef {
    group: String,
    kind: String,
    name: String,
}

