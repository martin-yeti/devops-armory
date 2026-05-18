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
    pub resourceVersion: Option<String>
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
    pub matches: Option<Vec<MatchCriteria>>,
    pub filters: Option<Vec<Filter>>,
    pub backendRefs: Vec<BackendRef>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchCriteria {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<Path>,
    pub headers: Option<Vec<NameMatch>>,
    #[serde(rename = "queryParams")]
    pub query_params: Option<Vec<NameMatch>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<MethodMatch>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Path {
    pub r#type: PathMatchType, 
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filter {    
    #[serde(rename = "type")]    
    pub filter_type: HttpRouteFilterType,    
    #[serde(default, skip_serializing_if = "Option::is_none")]    
    pub config: Option<FilterConfig>,
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

// Get resource version models

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HTTPRouteGetParent {
    pub items: Vec<MetadataGet>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HTTPRouteGet {
    pub metadata: MetadataGet,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MetadataGet {
    pub resourceVersion: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "spec", rename_all = "PascalCase")]
pub enum FilterConfig {
    RequestHeaderModifier(RequestHeaderModifier),
    ResponseHeaderModifier(ResponseHeaderModifier),
    RequestMirror(RequestMirror),
    RequestRedirect(RequestRedirect),
    UrlRewrite(UrlRewrite),
    FaultInjection(FaultInjection),
    RateLimit(RateLimit),
    Authentication(Authentication),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum HttpRouteFilterType {
    RequestHeaderModifier,
    ResponseHeaderModifier,
    RequestMirror,
    RequestRedirect,
    UrlRewrite,
    FaultInjection,
    RateLimit,
    Authentication,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestHeaderModifier {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add: Vec<HeaderOp>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remove: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub set: Vec<HeaderOp>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseHeaderModifier {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add: Vec<HeaderOp>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remove: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub set: Vec<HeaderOp>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderOp {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestMirror {
    pub backend_service: String,
    #[serde(default)]
    pub percentage: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestRedirect {
    pub scheme: Option<String>,
    pub host: Option<String>,
    pub path: Option<String>,
    pub port: Option<u16>,
    pub status_code: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlRewrite {
    pub host: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FaultInjection {
    pub delay_ms: Option<u64>,
    pub abort_http_status: Option<u16>,
    pub percentage: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RateLimit {
    pub requests_per_unit: Option<u64>,
    pub unit: Option<String>, // e.g., "second", "minute"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Authentication {
    pub provider: Option<String>,
    pub jwt_audiences: Option<Vec<String>>,
    pub jwks_uri: Option<String>,
}

/// HTTPRoute matches options

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PathMatchType {
    PathPrefix,
    PathExact,
    PathRegex,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NameMatch {
    #[serde(rename = "type")]
    pub kind: MatchType,
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MatchType {
    Exact,
    Prefix,
    RegularExpression,
}

// "GET", "POST" — keep as String for flexibility

#[derive(Debug, Serialize, Deserialize)]
pub struct MethodMatch {
    pub method: String,
}

