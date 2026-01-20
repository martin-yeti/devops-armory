use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct CreateDeployment {
    pub apiVersion: String,
    pub kind: String,
    pub metadata: DeploymentMetadata,
    pub spec: DeploymentSpec,
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct DeploymentMetadata {
    pub name: String,
    pub labels: DeploymentMetadataLabels,
    pub namespace: String
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct DeploymentMetadataLabels {
    pub app: String
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct DeploymentSpec {
    pub strategy: DeploymentSpecStrategy,
    pub replicas: i32,
    pub selector: DeploymentSelector,
    pub template: DeploymentTemplate,
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct DeploymentSpecStrategy {
    pub r#type: String,
    pub rollingUpdate: RollingUpdate
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct RollingUpdate {
    pub maxSurge: String,
    pub maxUnavailable: String,
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct DeploymentSelector {
    pub matchLabels: SelectorLabels
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct SelectorLabels {
    pub app: String
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct DeploymentTemplate {
    pub metadata: DeploymentTemplateMetadata,
    pub spec: DeploymentTemplateSpec
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct DeploymentTemplateMetadata {
    pub labels: DeploymentTemplateMetadataLabels
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct DeploymentTemplateMetadataLabels {
    pub app: String
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct DeploymentTemplateSpec {
    pub restartPolicy: String,
    pub dnsPolicy: String,
    pub terminationGracePeriodSeconds: i64,
    pub containers: Vec<DeploymentContainers>
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct DeploymentContainers {
    pub name: String,
    pub image: String,
    pub imagePullPolicy: String,
    pub command: Vec<String>,
    pub resources: DeploymentResources,
    pub ports: Vec<DeploymentPorts>,
    pub livenessProbe: DeploymentProbe,
    pub readinessProbe: DeploymentProbe,
    pub env: Vec<DeploymentEnvs>
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct DeploymentResources {
    pub requests: DeploymentResourcesRequests,
    pub limits: DeploymentResourcesLimits,
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct DeploymentResourcesRequests {
    pub memory: String,
    pub cpu: f32
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct DeploymentResourcesLimits {
    pub memory: String,
    pub cpu: f32
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct DeploymentProbe {
    pub failureThreshold: i32,
    pub httpGet: HttpProbe,
    pub initialDelaySeconds: i32,
    pub periodSeconds: i32,
    pub successThreshold: i8,
    pub timeoutSeconds: i32
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct HttpProbe {
    pub path: String,
    pub port: i64,
    pub scheme: String,
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct DeploymentPorts {
    pub containerPort: i64,
    pub protocol: String
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct DeploymentEnvs {
    pub name: String,
    pub value: String
}

// Update Deployment
// Handle with care !

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct UpdateDeployment {
    pub apiVersion: String,
    pub kind: String,
    pub metadata: DeploymentMetadata,
    pub spec: UpdateDeploymentSpec,
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct UpdateDeploymentSpec {
    pub selector: UpdateDeploymentSelector,
    pub template: UpdateDeploymentTemplate,
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct UpdateDeploymentSelector {
    pub matchLabels: UpdateSelectorLabels 
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct UpdateSelectorLabels {
    pub app: String
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct UpdateDeploymentTemplate {
    pub metadata: UpdateDeploymentTemplateMetadata,
    pub spec: UpdateDeploymentTemplateSpec
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct UpdateDeploymentTemplateMetadata {
    pub labels: UpdateDeploymentTemplateMetadataLabels
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct UpdateDeploymentTemplateMetadataLabels {
    pub app: String
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct UpdateDeploymentTemplateSpec {
    pub containers: Option<Vec<UpdateDeploymentContainers>>
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct UpdateDeploymentContainers {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub imagePullPolicy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub command: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]    
    pub resources: Option<DeploymentResources>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ports: Option<Vec<DeploymentPorts>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub livenessProbe: Option<DeploymentProbe>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub readinessProbe: Option<DeploymentProbe>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub env: Option<Vec<DeploymentEnvs>>
}

