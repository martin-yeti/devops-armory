use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Gateway {
    pub kind: String,
    pub apiVersion: String,
    pub metadata: Metadata,
    pub spec: GatewaySpec,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub namespace: String,
    pub resourceVersion: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GatewaySpec {
    pub gatewayClassName: String,
    pub listeners: Vec<Listener>,
    pub addresses: Vec<Address>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Listener {
    pub name: String,
    pub protocol: String,
    pub port: u16,
    pub allowedRoutes: AllowedRoutes,
    pub tls: Option<Tls>,
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
    pub from: From,
    pub selector: Option<Selector>
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

// Get resource version models

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GatewayGet {
    pub metadata: GatewayMetadataGet,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GatewayMetadataGet {
    pub resourceVersion: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum From {    
    All,    
    Selector,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Selector {    
    #[serde(rename = "matchLabels")]    
    pub match_labels: HashMap<String, String>,
}
