use std::collections::HashMap;

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CreateHetznerVolume {
    pub size: i32,
    pub name: String,
    pub labels: Option<HashMap<String, String>>,
    pub location: String,
    pub automount: Option<bool>,
    pub format: Option<String>,
    pub server: i64
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UpdateHetznerVolume {
    pub name: Option<String>,
    pub labels: Option<HashMap<String, String>>,
}
