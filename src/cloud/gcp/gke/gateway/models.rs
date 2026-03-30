use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Gateway {
    pub kind: String,
    pub api_version: String,
    pub metadata: Metadata,
    pub spec: GatewaySpec,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub namespace: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GatewaySpec {
    pub gateway_class_name: String,
    pub listeners: Vec<Listener>,
    pub addresses: Vec<Address>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Listener {
    pub name: String,
    pub protocol: String,
    pub port: u16,
    pub allowed_routes: AllowedRoutes,
    pub tls: Tls,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllowedRoutes {
    pub kinds: Vec<RouteKind>,
    pub namespaces: NamespaceSelector,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteKind {
    pub kind: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamespaceSelector {
    pub from: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tls {
    pub mode: String,
    pub options: TlsOptions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TlsOptions {
    #[serde(rename = "networking.gke.io/pre-shared-certs")]
    pub pre_shared_certs: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub r#type: String, 
    pub value: String,
}

