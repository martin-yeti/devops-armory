use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct Service {
    pub apiVersion: String,
    pub kind: String,
    pub metadata: ServiceMetadata,
    pub spec: ServiceSpecTemplateSpec
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct ServiceMetadata {
    pub name: String,
    pub namespace: String,
    pub annotations: ServiceAnnotations
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct ServiceSpec {
    pub template: ServiceSpecTemplate,
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct ServiceAnnotations {
    #[serde(rename = "cloud.google.com/backend-config")]
    pub cloud_google_com_backend_config: String,
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct ServiceSpecTemplate {
    pub spec: ServiceSpecTemplateSpec,
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct ServiceSpecTemplateSpec {
    pub selector: ServiceSpecTemplateSpecSelector,
    pub ports: Vec<ServiceSpecTemplateSpecPorts>
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct ServiceSpecTemplateSpecSelector {
    pub app: String,
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct ServiceSpecTemplateSpecPorts {
    pub name: String,
    pub protocol: String,
    pub port: i32,
    pub targetPort: i32
}
