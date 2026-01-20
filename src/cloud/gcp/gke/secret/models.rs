use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateSecret {
    pub apiVersion: String,
    pub kind: String,
    pub metadata: CreateMetadata,
    pub r#type: String,
    pub stringData: HashMap<String, String>
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateMetadata {
    pub name: String,
    pub namespace: String,
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct Secrets {
    pub items: Vec<SecretItems>
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct SecretItems {
    pub metadata: SecretMetadata,
    pub data: HashMap<String, String>
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct SecretMetadata {
    pub name: String,
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct SecretData {

}
