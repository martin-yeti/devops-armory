use actix_cors::Cors;
use actix_web::http;
use anyhow::Ok;
use serwus::web;

use serwus::server::Serwus;

use super::connection::prepare_app_data;
use super::handlers::filter_logs;

use super::insert_db::gke_log_collector_db;

/// Handlers function
pub fn config_app(app: &mut web::ServiceConfig<'_>) {

    // Log handler
    app.route(
        "/logs/{google_project_id}",
        web::get().to(filter_logs),
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
        .set_app_port("8888")
        .json_errors()
        .start(prepare_app_data, config_app, move || {
            cors_factory(&cors_allowed_origin)
    }).await.unwrap_or_default();

    Ok(server)

}

/// Wrapping function for inserting logs into DB
pub async fn collect_logs_db(
    token: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    gke_pod_list: &Vec<String>,
    gke_pod_phrase: &Vec<String>,
    project_name: String,
    gcp_id: String,
    gke_cluster_region: String,
) -> Result<(), anyhow::Error> {

    gke_log_collector_db(
                token,
                gke_cluster_endpoint,
                gke_cluster_namespace,
                gke_pod_list,
                gke_pod_phrase,
                project_name,
                gcp_id,
                gke_cluster_region,
    ).await.unwrap_or_default();

    Ok(())

}
