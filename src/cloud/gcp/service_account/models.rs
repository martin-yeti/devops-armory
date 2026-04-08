use serde::{Serialize, Deserialize};

#[derive(Debug,Deserialize, Serialize)]
pub struct CreateServiceAccount {
    pub accountId: String,
    pub serviceAccount: ServiceAccount
}

#[derive(Debug,Deserialize, Serialize)]
pub struct ServiceAccount {
    pub name: String,
    pub displayName: String,
    pub description: String,
    pub disabled: bool
}

#[derive(Debug,Deserialize, Serialize)]
pub struct UpdateServiceAccount {
    pub displayName: String,
}
