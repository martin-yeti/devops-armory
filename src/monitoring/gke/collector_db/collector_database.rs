use actix_cors::Cors;
use actix_web::http;
use anyhow::Ok;
use serwus::web;

use serwus::server::Serwus;

use super::connection::prepare_app_data;
use super::handlers::filter_pod_metrics;

use super::insert_db::gke_pod_metrics_collector_db;

/// Handlers function
pub fn config_app(app: &mut web::ServiceConfig<'_>) {

    // Pod metrics handler
    app.route(
        "/pod-metrics/{google_project_id}",
        web::get().to(filter_pod_metrics),
    );

}

/// CORS setup
/// Allowed origin is required
pub fn cors_factory(cors_allowed_origin: &str) -> Cors {
    let cors = Cors::default()
        .allowed_origin(cors_allowed_origin);

    let headers = vec![
        http::header::AUTHORIZATION,
        http::header::ACCEPT,
        http::header::CONTENT_TYPE,
    ];

    cors.supports_credentials()
        .allowed_methods(vec![
            "GET", "HEAD", "POST", "PATCH", "PUT", "DELETE", "OPTIONS",
        ])
        .allowed_headers(headers)
        .max_age(3600)
}

/// Server setup
/// Allowed origin parameter is required
pub async fn setup_server(
    cors_allowed_origin: String
) -> Result<(), anyhow::Error> {

    let server = Serwus::default()
        .set_app_port("8889")
        .json_errors()
        .start(prepare_app_data, config_app, move || {
            cors_factory(&cors_allowed_origin)
    }).await.unwrap_or_default();

    Ok(server)

}

/// Wrapping function for collecting pod cpu/ram metrics into DB
pub async fn collect_pod_metrics_db(
    token: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    gke_pod_phrase: &Vec<String>,
    project_name: String,
    gcp_id: String,
    gke_cluster_region: String,
    use_cgroup_v2: bool,
    poll_interval_secs: u64,
) -> Result<(), anyhow::Error> {

    gke_pod_metrics_collector_db(
                token,
                gke_cluster_endpoint,
                gke_cluster_namespace,
                gke_pod_phrase,
                project_name,
                gcp_id,
                gke_cluster_region,
                use_cgroup_v2,
                poll_interval_secs,
    ).await.unwrap_or_default();

    Ok(())

}
