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
    pub default: DefaultPolicy,
    pub targetRef: TargetRef,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogConfig {
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    #[serde(rename = "type")]
    pub kind: Option<ProtocolType>,
    #[serde(rename = "httpHealthCheck")]
    pub http_health_check: Option<HttpHealthCheck>,
    #[serde(rename = "httpsHealthCheck")]
    pub https_health_check: Option<HttpsHealthCheck>,
    #[serde(rename = "grpcHealthCheck")]
    pub grpc_health_check: Option<GrpcHealthCheck>,
    #[serde(rename = "http2HealthCheck")]
    pub http2_health_check: Option<Http2HealthCheck>,
    #[serde(rename = "tcpHealthCheck")]
    pub tcp_health_check: Option<TcpHealthCheck>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetRef {
    pub group: String,
    pub kind: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultPolicy {
    #[serde(rename = "checkIntervalSec")]
    pub check_interval_sec: Option<u32>,
    #[serde(rename = "timeoutSec")]
    pub timeout_sec: Option<u32>,
    #[serde(rename = "healthyThreshold")]
    pub healthy_threshold: Option<u32>,
    #[serde(rename = "unhealthyThreshold")]
    pub unhealthy_threshold: Option<u32>,
    #[serde(rename = "logConfig")]
    pub log_config: Option<LogConfig>,
    pub config: Option<HealthCheckConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProtocolType {
    HTTP,
    HTTPS,
    GRPC,
    HTTP2,
    TCP,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PortSpecification {
    USE_FIXED_PORT,
    USE_NAMED_PORT,
    USE_SERVING_PORT,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProxyHeader {
    NONE,
    PROXY_V1,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpHealthCheck {
    #[serde(rename = "portSpecification")]
    pub port_specification: Option<PortSpecification>,
    pub port: Option<u32>,
    pub host: Option<String>,
    #[serde(rename = "requestPath")]
    pub request_path: Option<String>,
    pub response: Option<String>,
    #[serde(rename = "proxyHeader")]
    pub proxy_header: Option<ProxyHeader>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpsHealthCheck {
    #[serde(rename = "portSpecification")]
    pub port_specification: Option<PortSpecification>,
    pub port: Option<u32>,
    pub host: Option<String>,
    #[serde(rename = "requestPath")]
    pub request_path: Option<String>,
    pub response: Option<String>,
    #[serde(rename = "proxyHeader")]
    pub proxy_header: Option<ProxyHeader>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrpcHealthCheck {
    pub grpc_service_name: Option<String>,
    pub port_specification: Option<PortSpecification>,
    pub port: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Http2HealthCheck {
    #[serde(rename = "portSpecification")]
    pub port_specification: Option<PortSpecification>,
    pub port: Option<u32>,
    pub host: Option<String>,
    #[serde(rename = "requestPath")]
    pub request_path: Option<String>,
    pub response: Option<String>,
    #[serde(rename = "proxyHeader")]
    pub proxy_header: Option<ProxyHeader>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TcpHealthCheck {
    #[serde(rename = "portSpecification")]
    pub port_specification: Option<PortSpecification>,
    pub port: Option<u32>,
    #[serde(rename = "portName")]
    pub port_name: Option<String>,
    pub request: Option<String>,
    pub response: Option<String>,
    #[serde(rename = "proxyHeader")]
    pub proxy_header: Option<ProxyHeader>,
}
