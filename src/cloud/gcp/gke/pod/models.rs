use serde_derive::Deserialize;

#[derive(Deserialize, Default,Debug)]
pub struct PodList {
    pub items: Vec<Items>
}

#[derive(Deserialize, Default,Debug,Clone)]
pub struct Items {
    pub metadata: Metadata
}

#[derive(Deserialize, Default,Debug, Clone)]
pub struct Metadata {
    pub name: String
}

