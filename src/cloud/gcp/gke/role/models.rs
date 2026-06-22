use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    pub kind: String,
    pub apiVersion: String,
    pub metadata: RoleMetadata,
    pub rules: Vec<RoleRules>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleMetadata {
    pub name: String,
    pub namespace: String,
    pub labels: HashMap<String, String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleRules {
    pub apiGroups: Vec<String>,
    pub resources: Vec<String>,
    pub verbs: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleBinding {
    pub kind: String,
    pub apiVersion: String,
    pub metadata: RoleMetadata,
    pub roleRef: RoleReference,
    pub subjects: Vec<RoleBindingSubjects>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleReference {
    pub apiGroup: String,
    pub kind: String,
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleBindingSubjects {
    pub king: String,
    pub name: String,
    pub namespace: String
}

