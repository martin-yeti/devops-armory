use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HTTPRoute {
    pub kind: String,
    pub apiVersion: String,
    pub metadata: Metadata,
    pub spec: HTTPRouteSpec,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub namespace: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HTTPRouteSpec {
    pub parentRefs: Vec<ParentRef>,
    pub hostnames: Vec<String>,  
    pub rules: Vec<Rule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentRef {
    pub kind: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    pub matches: Vec<MatchCriteria>,
    pub filters: Option<Vec<Filter>>,
    pub backendRefs: Vec<BackendRef>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchCriteria {
    pub path: Path,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Path {
    pub r#type: String, 
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filter {
    pub r#type: String,
    pub urlRewrite: URLRewrite,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct URLRewrite {
    pub hostname: String,
    pub path: PathRewrite,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PathRewrite {
    pub r#type: String,
    pub replacePrefixMatch: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackendRef {
    pub name: String,
    pub port: u16,
}

