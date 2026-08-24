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
    pub podIP: String,
    pub startTime: String,
    pub containerStatuses: Vec<Ready>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Ready {
    pub name: String,
    pub state: PodState,
    pub ready: bool,
    pub restartCount: i16,
    pub image: String,
    pub imageID: String,
    pub started: bool,
    pub allocatedResources: PodAllocatedResources,
    pub resources: PodResources
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct PodState {
    pub running: PodStateRunning
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct PodStateRunning {
    pub startedAt: String
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct PodAllocatedResources {
    pub cpu: String,
    pub memory: String
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct PodResources {
    pub limits: PodAllocatedResources,
    pub requests: PodAllocatedResources
}

