use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HTTPRoute {
    kind: String,
    api_version: String,
    metadata: Metadata,
    spec: HTTPRouteSpec,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    name: String,
    namespace: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HTTPRouteSpec {
    parent_refs: Vec<ParentRef>,
    hostnames: Vec<String>,  
    rules: Vec<Rule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentRef {
    kind: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    matches: Vec<MatchCriteria>,
    filters: Vec<Filter>,
    backend_refs: Vec<BackendRef>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchCriteria {
    path: Path,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Path {
    r#type: String, 
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filter {
    r#type: String,
    url_rewrite: URLRewrite,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct URLRewrite {
    hostname: String,
    path: PathRewrite,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PathRewrite {
    r#type: String,
    replace_prefix_match: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackendRef {
    name: String,
    port: u16,
}

