use std::collections::HashMap;

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct PersistentVolumeClaim {
    pub apiVersion: String,
    pub kind: String,
    pub metadata: PVCMetadata,
    pub spec: PVCSpec,
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct PVCMetadata {
    pub name: String,
    pub namespace: String,
    pub labels: Option<HashMap<String, String>>
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct PVCSpec {
    pub storageClassName: String,
    pub accessModes: Vec<String>,
    pub resources: PVCResources
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct PVCResources {
    pub requests: PVCRequests
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct PVCRequests {
    pub storage: String
}

