use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Ingress {
    pub apiVersion: String,
    pub kind: String,
    pub metadata: IngressMetadata,
    pub spec: IngressSpecTemplateSpec
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct IngressMetadata {
    pub name: String,
    pub namespace: String,
    pub annotations: Option<IngressMetadataAnnotations>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct IngressMetadataAnnotations {
    #[serde(rename = "kubernetes.io/ingress.global-static-ip-name")]
    pub kubernetes_io_ingress_global_static_ip_name: Option<String>,
    #[serde(rename = "networking.gke.io/managed-certificates")]
    pub networking_gke_io_managed_certificates: Option<String>,
    #[serde(rename = "kubernetes.io/ingress.class")]
    pub kubernetes_io_ingress_class: Option<String>,
    #[serde(rename = "nginx.ingress.kubernetes.io/force-ssl-redirect")]
    pub nginx_force_ssl_redirect: Option<String>,
    #[serde(rename = "cert-manager.io/cluster-issuer")]
    pub cert_manager_cluster_issuer: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct IngressSpecTemplateSpec {
    pub tls: Option<Vec<IngressSpecTemplateTlsRules>>,
    pub rules: Vec<IngressSpecTemplateSpecRules>
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct IngressSpecTemplateTlsRules {
    pub host: Vec<String>,
    pub secretName: String
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct IngressSpecTemplateSpecRules {
    pub host: String,
    pub http: SpecHttp
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SpecHttp {
    pub paths: Vec<Paths>
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Paths {
    pub path: String,
    pub pathType: String,
    pub backend: IngressBackend
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct IngressBackend {
    pub service: IngressBackendService
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct IngressBackendService {
    pub name: String,
    pub port: BackendPort
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct BackendPort {
    pub number: i32
}


