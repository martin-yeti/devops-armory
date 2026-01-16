use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct GcpProjectIam {
    pub policy: SetPolicy,
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct SetPolicy {
    pub bindings: Vec<PolicyBindings>,
    pub etag: String,
    pub version: i8
}

#[derive(Serialize, Deserialize, Default,Debug, PartialEq, Eq, Hash)]
pub struct PolicyBindings {
    pub role: String,
    pub members: Vec<String>,
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct MembersConditions {
    pub title: String,
    pub description: String,
    pub expression: String
}
