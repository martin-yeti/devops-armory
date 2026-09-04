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
    pub phase: String,
    pub conditions: Option<Vec<ContainerConditions>>,
    // Absent for pods that never got scheduled/assigned an IP, e.g. Evicted
    pub podIP: Option<String>,
    pub startTime: String,
    // Absent for pods that never had a container start, e.g. Evicted
    pub containerStatuses: Option<Vec<Ready>>,
    pub message: Option<String>,
    pub reason: Option<String>
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct ContainerConditions {
    pub r#type: String,
    pub status: String,
    pub reason: Option<String>,
    pub message: Option<String>,
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
    // Both absent once a container has exited, e.g. terminated/Evicted
    pub allocatedResources: Option<PodAllocatedResources>,
    pub resources: Option<PodResources>
}

// A container's state is a Kubernetes oneof: exactly one of running/waiting/terminated is set
#[derive(Debug, Deserialize, Default, Clone)]
pub struct PodState {
    pub running: Option<PodStateRunning>,
    pub waiting: Option<PodStateWaiting>,
    pub terminated: Option<PodStateTerminated>
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct PodStateRunning {
    pub startedAt: String
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct PodStateWaiting {
    pub reason: Option<String>,
    pub message: Option<String>
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct PodStateTerminated {
    pub exitCode: i32,
    pub reason: Option<String>,
    pub message: Option<String>,
    pub startedAt: Option<String>,
    pub finishedAt: Option<String>
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

