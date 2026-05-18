use std::collections::HashMap;

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CreateHetznerVM {
    pub name: String,
    pub server_type: String,
    pub image: String,
    pub location: String,
    pub ssh_keys: Vec<String>,
    pub start_after_create: Option<bool>,
    pub labels: Option<HashMap<String, String>>,
    pub user_data: Option<String>
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UpdateHetznerVM {
    pub name: String,
    pub labels: Option<HashMap<String, String>>
}
