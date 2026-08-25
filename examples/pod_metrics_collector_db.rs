// This example shows how to create a pod cpu/ram metrics collector with cors address passed to server
// Default port is 8889

use std::time::Duration;

use devops_armory::monitoring::gke::collector_db::collector_database::{
    setup_server,
    collect_pod_metrics_db
};

async fn collect_pod_metrics_function() -> Result<(), std::io::Error> {

    rustls::crypto::ring::default_provider().install_default().expect("Failed to install rustls crypto provider");

    // Set pod name phrase(s) to collect metrics from
    let v_pod_names = vec![
        "pod_name_phrase_to_include".to_string()
    ];

    // Collect cpu/ram metrics for pods matching the phrase(s) above, on a 60s poll interval,
    // reading from cgroup v2 (set to false for nodepools still on cgroup v1)
    collect_pod_metrics_db(
        "some_token".to_string(),
        "gke_cluster_address".to_string(),
        "gke_cluster_namespace".to_string(),
        &v_pod_names,
        "project_id".to_string(),
        "gcp_project_id".to_string(),
        "region".to_string(),
        true,
        60,
    ).await.unwrap_or_default();

    Ok(())

}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {

    // Server is bound once for the process lifetime; starting it again on
    // every collection cycle would try to rebind the already-open port.
    tokio::spawn(setup_server("cors_address_passed_to_server_api".to_string()));

    loop {

        // Re-fetch the token and restart the collector every 59 minutes,
        // since the underlying token expires.
        tokio::select! {
            _ = collect_pod_metrics_function() => {}
            _ = tokio::time::sleep(Duration::from_secs(59 * 60)) => {}
        }
    }
}
