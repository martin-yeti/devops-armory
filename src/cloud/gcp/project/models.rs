use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct GcpProjectList {
    pub projects: Vec<GcpProject>,
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct GcpProject {
    pub name: String,
    pub parent: String,
    pub projectId: String,
    pub state: String,
    pub displayName: String,
}

#[derive(Deserialize,Default,Debug)]
pub enum GcpProjectState {
    STATE_UNSPECIFIED,
    #[default] ACTIVE,
    DELETE_REQUESTED,
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct GcpProjectCreate {
    pub parent: String,
    pub projectId: String,
    pub state: String,
    pub displayName: String,
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct BillingInfo {
    pub billingAccountName: String,
}

