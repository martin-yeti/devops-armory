use serde::{Serialize, Deserialize};

#[derive(Debug,Deserialize, Serialize)]
pub struct CreateDNSRecord {
    pub name: String,
    pub r#type: String,
    pub ttl: i32,
    pub rrdatas: Vec<String>,
    pub signatureRrdatas: Option<Vec<String>>,
    pub kind: String, 
}


