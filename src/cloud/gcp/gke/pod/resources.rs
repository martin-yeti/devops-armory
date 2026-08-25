use std::time::Duration;

use awc::{
    Client, Connector,
};

use openssl::ssl::{
    SslConnector,
    SslMethod,
    SslVerifyMode
};

use super::models::PodName;

/// Parse a Kubernetes CPU quantity (e.g. "500m", "2") into cores
fn parse_cpu_quantity(quantity: &str) -> f64 {
    let quantity = quantity.trim();

    match quantity.strip_suffix('m') {
        Some(millicores) => millicores.parse::<f64>().unwrap_or_default() / 1000.0,
        None => quantity.parse::<f64>().unwrap_or_default(),
    }
}

/// Parse a Kubernetes memory quantity (e.g. "512Mi", "1Gi", "1000000") into mebibytes
fn parse_memory_quantity_mib(quantity: &str) -> f64 {
    let quantity = quantity.trim();

    let (value, divisor) = if let Some(v) = quantity.strip_suffix("Ki") {
        (v, 1.0 / 1024.0)
    } else if let Some(v) = quantity.strip_suffix("Mi") {
        (v, 1.0)
    } else if let Some(v) = quantity.strip_suffix("Gi") {
        (v, 1024.0)
    } else if let Some(v) = quantity.strip_suffix("Ti") {
        (v, 1024.0 * 1024.0)
    } else if let Some(v) = quantity.strip_suffix('K') {
        (v, 1000.0 / 1024.0 / 1024.0)
    } else if let Some(v) = quantity.strip_suffix('M') {
        (v, 1000.0 * 1000.0 / 1024.0 / 1024.0)
    } else if let Some(v) = quantity.strip_suffix('G') {
        (v, 1000.0 * 1000.0 * 1000.0 / 1024.0 / 1024.0)
    } else {
        (quantity, 1.0 / 1024.0 / 1024.0)
    };

    value.parse::<f64>().unwrap_or_default() * divisor
}

/// Requested/limit CPU (cores) and memory (mebibytes), summed across all
/// containers of a pod, plus its overall readiness
#[derive(Debug, Clone, Default)]
pub struct PodResourceInfo {
    pub requested_cpu: f64,
    pub requested_memory: f64,
    pub limit_cpu: f64,
    pub limit_memory: f64,
    pub healthy: bool,
}

/// Get requested/limit CPU and memory plus readiness for a pod, summed
/// across all of its containers
/// Token, gke endpoint, namespace and pod name need to be provided
pub async fn get_pod_resource_info(
    token: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    gke_pod_name: String
) -> Result<PodResourceInfo, std::io::Error> {

    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE);
    let myconnector = builder.build();
    let client = Client::builder()
        .connector(Connector::new().openssl(myconnector))
        .finish();

    let mut response = client
        .get(format!("https://{gke_cluster_endpoint}:443/api/v1/namespaces/{gke_cluster_namespace}/pods/{gke_pod_name}"))
        .bearer_auth(format!("{token}"))
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to get pod {gke_pod_name}: {err}")))?;

    let pod_info = response
        .json::<PodName>()
        .await
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to parse pod response: {err}")))?;

    let containers = &pod_info.status.containerStatuses;

    Ok(PodResourceInfo {
        requested_cpu: containers.iter().map(|c| parse_cpu_quantity(&c.resources.requests.cpu)).sum(),
        requested_memory: containers.iter().map(|c| parse_memory_quantity_mib(&c.resources.requests.memory)).sum(),
        limit_cpu: containers.iter().map(|c| parse_cpu_quantity(&c.resources.limits.cpu)).sum(),
        limit_memory: containers.iter().map(|c| parse_memory_quantity_mib(&c.resources.limits.memory)).sum(),
        healthy: !containers.is_empty() && containers.iter().all(|c| c.ready),
    })

}

/// Get the total CPU requested (in cores) across all containers of a pod
/// Token, gke endpoint, namespace and pod name need to be provided
pub async fn get_pod_requested_cpu(
    token: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    gke_pod_name: String
) -> Result<f64, std::io::Error> {

    Ok(get_pod_resource_info(token, gke_cluster_endpoint, gke_cluster_namespace, gke_pod_name).await?.requested_cpu)

}

/// Get the total memory requested (in mebibytes) across all containers of a pod
/// Token, gke endpoint, namespace and pod name need to be provided
pub async fn get_pod_requested_memory(
    token: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    gke_pod_name: String
) -> Result<f64, std::io::Error> {

    Ok(get_pod_resource_info(token, gke_cluster_endpoint, gke_cluster_namespace, gke_pod_name).await?.requested_memory)

}
