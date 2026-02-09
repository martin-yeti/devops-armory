use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct SqlDb {
    pub charset: String,
    pub collation: String,
    pub instance: String,
    pub project: String,
    pub name: String
}
