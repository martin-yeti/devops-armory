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
    pub annotations: IngressMetadataAnnotations,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct IngressMetadataAnnotations {
    #[serde(rename = "kubernetes.io/ingress.global-static-ip-name")]
    pub kubernetes_io_ingress_global_static_ip_name: String,
    #[serde(rename = "networking.gke.io/managed-certificates")]
    pub networking_gke_io_managed_certificates: String,
    //#[serde(rename = "kubernetes.io/ingress.class")]
    //pub kubernetes_io_ingress_class: String
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct IngressSpecTemplateSpec {
    pub rules: Vec<IngressSpecTemplateSpecRules>
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


