use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct SqlUser {
    pub password: String,
    pub name: String,
    pub instance: String,
    pub project: String
}
