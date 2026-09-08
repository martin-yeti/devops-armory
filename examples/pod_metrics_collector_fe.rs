// This example shows how to run the pod metrics web UI
// It queries the pod-metrics collector API (see examples/pod_metrics_collector_db.rs)
// Default port is 8081

use devops_armory::monitoring::gke::collector_fe::collector_fe::collector_fe;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Base URL of the pod-metrics collector API started by
    // examples/pod_metrics_collector_db.rs (default port 8889)
    let api_base_url = "http://127.0.0.1:8889/pod-metrics".to_string();

    collector_fe(api_base_url).await
}
