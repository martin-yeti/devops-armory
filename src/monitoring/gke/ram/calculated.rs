use super::inactive_file::{
    mem_inactive_file_cgroup1,
    mem_inactive_file_cgroup2
};

use super::usage::{
    mem_usage_in_bytes_cgroup1,
    mem_usage_in_bytes_cgroup2
};

/// RAM usage for specific pod in CGROUP2
/// Token, GKE endpoint, namespace, pod name and RAM requested need to be provided
pub async fn mem_calculated_cgroup2(
    token: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    gke_pod_name: String,
    ram: f64
) -> Result<f64, std::io::Error> {

    //let mem_inactive_file = mem_inactive_file_cgroup2(
    //    token.clone(),
    //    gke_cluster_endpoint.clone(),
    //    gke_cluster_namespace.clone(),
    //    gke_pod_name.clone()
    //).await.unwrap();

    let mem_usage_in_bytes = mem_usage_in_bytes_cgroup2(
        token.clone(),
        gke_cluster_endpoint.clone(),
        gke_cluster_namespace.clone(),
        gke_pod_name.clone()
    ).await.unwrap();

    loop {
        //let mem_usage: f64 = (mem_usage_in_bytes - mem_inactive_file) / 1024.0 / 1024.0;
        let ram_usage_cgroup2 = (mem_usage_in_bytes / 1024.0 / 1024.0) * 100.0 / ram;

        return Ok(ram_usage_cgroup2);
    }

}

/// RAM usage for specific pod in CGROUP1
/// Token, GKE endpoint, namespace, pod name and RAM requested need to be provided
pub async fn mem_calculated_cgroup1(
    token: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    gke_pod_name: String,
    ram: f64
) -> Result<f64, std::io::Error> {

    //let mem_inactive_file = mem_inactive_file_cgroup1(
    //    token.clone(),
    //    gke_cluster_endpoint.clone(),
    //    gke_cluster_namespace.clone(),
    //    gke_pod_name.clone()
    //).await.unwrap();

    let mem_usage_in_bytes = mem_usage_in_bytes_cgroup1(
        token.clone(),
        gke_cluster_endpoint.clone(),
        gke_cluster_namespace.clone(),
        gke_pod_name.clone()
    ).await.unwrap();

    loop {
        //let mem_usage: f64 = ((mem_usage_in_bytes - mem_inactive_file) / 1024.0 / 1024.0) / ram;
        let ram_usage_cgroup1 = (mem_usage_in_bytes / 1024.0 / 1024.0) * 100.0 / ram;

        return Ok(ram_usage_cgroup1);
    }

}
