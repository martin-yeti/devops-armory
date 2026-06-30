use actix_cors::Cors;
use actix_web::http;
use serwus::web;

use serwus::server::Serwus;

use super::connection::prepare_app_data;
use super::handlers::filter_logs;

use crate::logging::gke::collector_stdout::gke_log_collector;

fn config_app(app: &mut web::ServiceConfig<'_>) {

    // Log handler
    app.route(
        "/logs/{google_project_id}",
        web::get().to(filter_logs),
    );

}

fn cors_factory() -> Cors {
    let cors = Cors::default()
        .allowed_origin("http://localhost:4444");

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

pub async fn collect_logs_db(
    token: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    gke_pod_list: &Vec<String>,
    gke_pod_phrase: &Vec<String>,
    project_name: String,
    project_region: String,
    gcp_id: String,
    gke_cluster_region: String,
) -> std::io::Result<()> {

    let server = Serwus::default()
        .set_app_port("8888")
        .json_errors()
        .start(prepare_app_data, config_app, cors_factory);

    tokio::select! {
        _ = server => {}
        _ = gke_log_collector(
                token,
                gke_cluster_endpoint,
                gke_cluster_namespace,
                gke_pod_list,
                gke_pod_phrase,
                project_name,
                project_region,
                gcp_id,
                gke_cluster_region,
            ) => {}
    }

    Ok(())

}
