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

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct GetSSL {
    pub kind: String,
    pub id: String,
    pub creationTimestamp: String,
    pub name: String,
    pub description: String,
    pub selfLink: String,
    pub certificate: String,
    pub managed: GetManagedDomains,
    pub r#type: String,
    pub subjectAlternativeNames: Vec<String>,
    pub expireTime: String
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct GetManagedDomains {
    pub domains: Vec<String>,
    pub status: String,
    //pub domainStatus: HashMap<String, String>
}
