use std::{
    thread::
    sleep,
    time
};

use crate::cloud::gcp::gke::pod::resources::get_pod_requested_cpu;

use super::usage::{
    cpu_usage_cfgroup2,
    cpu_usage_cfgroup1
};

/// Function to calculate used cpu - based on cpu cycles on cfgroup2 config in nodepool
/// Amount of requested CPU is fetched from the pod's resource requests
pub async fn cpu_calculated_cfgroup2(
    token: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    gke_pod_name: String
) -> Result<f64, std::io::Error> {

    let cpu = get_pod_requested_cpu(
        token.clone(),
        gke_cluster_endpoint.clone(),
        gke_cluster_namespace.clone(),
        gke_pod_name.clone())
        .await?;

    let cpu_use_old = cpu_usage_cfgroup2(
        token.clone(),
        gke_cluster_endpoint.clone(),
        gke_cluster_namespace.clone(),
        gke_pod_name.clone())
        .await?;

    let cpu_use_new = cpu_usage_cfgroup2(
        token.clone(),
        gke_cluster_endpoint.clone(),
        gke_cluster_namespace.clone(),
        gke_pod_name.clone())
    .await?;

    loop {
        let cpu_1 = cpu_use_old;

        sleep(time::Duration::from_millis(1000));

        let cpu_2 = cpu_use_new;
        let cpu_actual_usage = (cpu_2 - cpu_1) / 1000000.0 / cpu;

        return Ok(cpu_actual_usage);
    }

}


/// Function to calculate used cpu - based on cpu cycles on cfgroup1 config in nodepool
/// Amount of requested CPU is fetched from the pod's resource requests
pub async fn cpu_calculated_cfgroup1(
    token: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    gke_pod_name: String
) -> Result<f64, std::io::Error> {

    let cpu = get_pod_requested_cpu(
        token.clone(),
        gke_cluster_endpoint.clone(),
        gke_cluster_namespace.clone(),
        gke_pod_name.clone())
        .await?;

    let cpu_use_old = cpu_usage_cfgroup1(
        token.clone(),
        gke_cluster_endpoint.clone(),
        gke_cluster_namespace.clone(),
        gke_pod_name.clone())
    .await?;

    let cpu_use_new = cpu_usage_cfgroup1(
        token.clone(),
        gke_cluster_endpoint.clone(),
        gke_cluster_namespace.clone(),
        gke_pod_name.clone())
    .await?;

    loop {
        let cpu_1 = cpu_use_old;

        sleep(time::Duration::from_millis(1000));

        let cpu_2 = cpu_use_new;
        let cpu_actual_usage = (cpu_2 - cpu_1) / 1000000.0 / cpu;

        return Ok(cpu_actual_usage);
    }
}
