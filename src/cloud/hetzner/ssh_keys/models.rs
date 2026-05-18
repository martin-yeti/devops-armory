use std::collections::HashMap;

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CreateHetznerSSH {
    pub name: String,
    pub public_key: String,
    pub labels: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UpdateHetznerSSH {
    pub name: String,
    pub labels: Option<HashMap<String, String>>,
}
