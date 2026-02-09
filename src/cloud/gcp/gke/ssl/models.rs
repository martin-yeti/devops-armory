use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct GkeSslCert {
    pub apiVersion: String,
    pub kind: String,
    pub metadata: SslMetadata,
    pub spec: SslSpec
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SslMetadata {
    pub name: String,
    pub namespace: String
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SslSpec {
    pub domains: Vec<String>
}

