use std::collections::HashMap;

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct ConfigMap {
    pub apiVersion: String,
    pub kind: String,
    pub metadata: ConfigMapMetadata,
    pub data: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct ConfigMapMetadata {
    pub name: String,
    pub namespace: String,
    pub labels: Option<HashMap<String, String>>
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct UpdateConfigMap {
    pub metadata: ConfigMapMetadata,
}
