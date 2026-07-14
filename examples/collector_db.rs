// This example shows how to create log collector with cors address passed to server
// Deault port is 8888

use std::time::Duration;

use devops_armory::logging::gke::collector_db::collector_database::{
    setup_server
};
use devops_armory::cloud::gcp::gke::pod::list::pod_list_vector;
use devops_armory::logging::gke::collector_db::collector_database::collect_logs_db;



async fn collect_logs_function() -> Result<(), std::io::Error> {

    rustls::crypto::ring::default_provider().install_default().expect("Failed to install rustls crypto provider");

    // Prepare new empty vector for pod list
    let mut v: Vec<String> = Vec::new();

    // Get pods info
    let pod_list = pod_list_vector(
        "some_token".to_string(), 
        "gke_cluster_address".to_string(), 
        "gke_cluster_namespace".to_string()
    ).await.unwrap_or_default();

    // Iterate over pod_list vector and push pod names into new one - v
    for a in pod_list {
        let b = a.metadata.name;
        v.push(b);
    }

    // Set pod name to collect logs from
    let v_pod_names = vec![
        "pod_name_phrase_to_include".to_string()
    ];

    // Collect logs consisting provided name(s)
    let logs = collect_logs_db(
        "some_token".to_string(), 
        "gke_cluster_address".to_string(), 
        "gke_cluster_namespace".to_string(),
        &v,
        &v_pod_names,
        "project_id".to_string(),
        "gcp_project_id".to_string(),
        "region".to_string(),
    ).await.unwrap_or_default();

    Ok(logs)

}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {

    // Server is bound once for the process lifetime; starting it again on
    // every collection cycle would try to rebind the already-open port.
    tokio::spawn(setup_server("cors_address_passed_to_server_api".to_string()));

    loop {

        // Re-fetch the token and restart the collectors every 59 minutes,
        // since collect_logs_db streams indefinitely and the token expires.
        tokio::select! {
            _ = collect_logs_function() => {}
            _ = tokio::time::sleep(Duration::from_secs(59 * 60)) => {}
        }
    }
}
