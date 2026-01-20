use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct FrontendConfig {
    pub apiVersion: String,
    pub kind: String,
    pub metadata: FrontendConfigMetadata,
    pub spec: FrontendConfigSpec
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct FrontendConfigMetadata {
    pub name: String,
    pub namespace: String
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct FrontendConfigSpec {
    pub redirectToHttps: HttpRedirect
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct HttpRedirect {
    pub enabled: bool,
    pub responseCodeName: String
}

