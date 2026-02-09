use serde_derive::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct CreateSSL {
    pub name: String,
    pub managed: DomainsManaged,
    pub r#type: String,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct DomainsManaged {
    pub domains: Vec<String>,
}
