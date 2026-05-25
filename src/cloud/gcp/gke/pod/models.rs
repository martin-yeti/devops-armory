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


#[derive(Debug, Deserialize, Default, Clone)]
pub struct PodName {
    pub status: ContainerStatuses,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct ContainerStatuses {
    pub containerStatuses: Vec<Ready>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Ready {
    pub ready: bool,
}
