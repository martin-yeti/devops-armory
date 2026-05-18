use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct CreateNamespace {
    pub apiVersion: String,
    pub kind: String,
    pub metadata: NamespaceMetadata,
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct NamespaceMetadata {
    pub name: String,
    pub labels: Option<NamespaceLabels>
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct NamespaceLabels {
    pub otherInfra: String,
}

