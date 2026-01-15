use std::collections::HashMap;
use serde::Deserialize;

#[derive(Default, Debug, Deserialize)]
pub struct Root {
    pub rustible: Vec<Rustible>,
    pub cloud: Vec<Cloud>
}

#[derive(Default, Debug, Deserialize)]
pub struct Rustible {
    pub vm: Vec<Vm>,
    pub bare_metal: Vec<BareMetal>,
}

#[derive(Default, Debug, Deserialize)]
pub struct Vm {
    pub ubuntu: VMSpecs,
    pub slackware: VMSpecs,
    pub debian: VMSpecs,
    pub centos: VMSpecs,
    pub fedora: VMSpecs,
    pub opensuse: VMSpecs
}

#[derive(Default, Debug, Deserialize)]
pub struct VMSpecs {
    pub ip_address_list: Vec<String>,
    pub commands: Vec<String>,
}

#[derive(Default, Debug, Deserialize)]
pub struct BareMetal {
    pub ubuntu: BareMetalSpecs,
    pub debian: BareMetalSpecs,
    pub slackware: BareMetalSpecs,
    pub centos: BareMetalSpecs,
    pub fedora: BareMetalSpecs,
    pub opensuse: BareMetalSpecs
}

#[derive(Default, Debug, Deserialize)]
pub struct BareMetalSpecs {
    pub ip_address_list: Vec<String>,
    pub commands: Vec<String>,
}

#[derive(Default, Debug, Deserialize)]
pub struct Cloud {
    // This allows dynamic keys directly in the struct
    #[serde(flatten)] 
    pub projects: HashMap<String, Vec<CloudProject>>, 
}

#[derive(Default, Debug, Deserialize)]
pub struct CloudProject {
    pub name: String,
    pub address_k8s: Vec<String>,
    pub credentials: String
}

