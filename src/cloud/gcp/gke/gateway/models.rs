use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Gateway {
    kind: String,
    api_version: String,
    metadata: Metadata,
    spec: GatewaySpec,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    name: String,
    namespace: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GatewaySpec {
    gateway_class_name: String,
    listeners: Vec<Listener>,
    addresses: Vec<Address>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Listener {
    name: String,
    protocol: String,
    port: u16,
    allowed_routes: AllowedRoutes,
    tls: Tls,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllowedRoutes {
    kinds: Vec<RouteKind>,
    namespaces: NamespaceSelector,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteKind {
    kind: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamespaceSelector {
    from: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tls {
    mode: String,
    options: TlsOptions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TlsOptions {
    #[serde(rename = "networking.gke.io/pre-shared-certs")]
    pre_shared_certs: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    r#type: String, 
    value: String,
}

