use std::time::Duration;

use diesel::insert_into;
use diesel::RunQueryDsl;

use futures::stream::{self, StreamExt};

use log::{info, warn, error};

use serwus::db_pool::async_write_transaction;
use serwus::db_pool::multi::MultiPoolBuilder;

use crate::cloud::gcp::gke::pod::list::pod_list_vector;
use crate::cloud::gcp::gke::pod::resources::get_pod_resource_info;
use crate::monitoring::gke::cpu::calculated::{cpu_calculated_cfgroup1, cpu_calculated_cfgroup2};
use crate::monitoring::gke::ram::calculated::{mem_calculated_cgroup1, mem_calculated_cgroup2};

use super::models::NewPodMetric;
use super::schema::pod_metrics::dsl::pod_metrics;

/// Number of pods collected concurrently per polling tick.
/// Matches the DB pool size: raising this past the pool size just parks
/// extra blocking-pool threads waiting on a connection checkout, with no
/// added throughput.
const MAX_CONCURRENT_COLLECTIONS: usize = 4;

fn round_2dp(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

async fn measure_usage(
    token: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    pod_name: String,
    use_cgroup_v2: bool,
) -> (Option<f64>, Option<f64>) {
    let (cpu_usage_result, ram_usage_result) = if use_cgroup_v2 {
        tokio::join!(
            cpu_calculated_cfgroup2(token.clone(), gke_cluster_endpoint.clone(), gke_cluster_namespace.clone(), pod_name.clone()),
            mem_calculated_cgroup2(token.clone(), gke_cluster_endpoint.clone(), gke_cluster_namespace.clone(), pod_name.clone()),
        )
    } else {
        tokio::join!(
            cpu_calculated_cfgroup1(token.clone(), gke_cluster_endpoint.clone(), gke_cluster_namespace.clone(), pod_name.clone()),
            mem_calculated_cgroup1(token.clone(), gke_cluster_endpoint.clone(), gke_cluster_namespace.clone(), pod_name.clone()),
        )
    };

    let cpu_usage = match cpu_usage_result {
        Ok(usage) => Some(round_2dp(usage)),
        Err(err) => {
            error!("Failed to get cpu usage for pod {}: {}", pod_name, err);
            None
        }
    };

    let ram_usage = match ram_usage_result {
        Ok(usage) => Some(round_2dp(usage)),
        Err(err) => {
            error!("Failed to get ram usage for pod {}: {}", pod_name, err);
            None
        }
    };

    (cpu_usage, ram_usage)
}

/// Poll GKE for pod resource requests/limits, health and live cpu/ram usage,
/// then insert one row per pod into the database on every tick.
/// Token, cluster endpoint, namespace and pod name phrase filter need to be
/// provided. `use_cgroup_v2` selects which cgroup version's usage functions
/// to read from (cgroup v2 is the default on current GKE node images).
/// `poll_interval_secs` sets how often the pod list is re-scanned.
///
/// Collection (GET + exec) runs on the calling task via `for_each_concurrent`
/// rather than `tokio::spawn`, since `awc::Client` futures hold an `Rc`
/// internally and are not `Send`; only the DB insert, which is `Send`-safe,
/// is spawned onto its own task.
pub async fn gke_pod_metrics_collector_db(
    token: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    gke_pod_phrase: &Vec<String>,
    project_name: String,
    gcp_id: String,
    gke_cluster_region: String,
    use_cgroup_v2: bool,
    poll_interval_secs: u64,
) -> Result<(), std::io::Error> {

    let db_pool = MultiPoolBuilder::default()
        .size(MAX_CONCURRENT_COLLECTIONS)
        .connect()
        .expect("Can't connect to database");

    let mut ticker = tokio::time::interval(Duration::from_secs(poll_interval_secs));

    loop {
        ticker.tick().await;

        let pods = match pod_list_vector(
            token.clone(),
            gke_cluster_endpoint.clone(),
            gke_cluster_namespace.clone(),
        ).await {
            Ok(pods) => pods,
            Err(err) => {
                error!("Failed to list pods, skipping this tick: {}", err);
                continue;
            }
        };

        let matched_pods: Vec<String> = pods
            .into_iter()
            .map(|pod| pod.metadata.name)
            .filter(|pod_name| gke_pod_phrase.iter().any(|phrase| pod_name.contains(phrase)))
            .collect();

        if matched_pods.is_empty() {
            warn!("No matching pods this tick. Retrying in {}s", poll_interval_secs);
            continue;
        }

        stream::iter(matched_pods)
            .for_each_concurrent(MAX_CONCURRENT_COLLECTIONS, |pod_name| {
                let pool = db_pool.clone();
                let token = token.clone();
                let gke_cluster_endpoint = gke_cluster_endpoint.clone();
                let gke_cluster_namespace = gke_cluster_namespace.clone();
                let project_name = project_name.clone();
                let gcp_id = gcp_id.clone();
                let gke_cluster_region = gke_cluster_region.clone();

                async move {
                    let resource_info = match get_pod_resource_info(
                        token.clone(),
                        gke_cluster_endpoint.clone(),
                        gke_cluster_namespace.clone(),
                        pod_name.clone(),
                    ).await {
                        Ok(info) => info,
                        Err(err) => {
                            error!("Failed to get resource info for pod {}: {}", pod_name, err);
                            return;
                        }
                    };

                    // An unhealthy pod has no running container to exec into - live
                    // usage is unmeasurable by definition, and attempting it risks
                    // hanging on a dead pod's exec session. Skip straight to a row
                    // with cpu_usage/ram_usage left unset instead of dropping it.
                    let (cpu_usage, ram_usage) = if resource_info.healthy {
                        measure_usage(
                            token.clone(),
                            gke_cluster_endpoint.clone(),
                            gke_cluster_namespace.clone(),
                            pod_name.clone(),
                            use_cgroup_v2,
                        ).await
                    } else {
                        (None, None)
                    };

                    let new_metric = NewPodMetric {
                        google_project_id: gcp_id,
                        project_id: project_name,
                        region: gke_cluster_region,
                        namespace: gke_cluster_namespace,
                        pod_name: pod_name.clone(),
                        cpu_request: resource_info.requested_cpu,
                        ram_request: resource_info.requested_memory,
                        cpu_limit: resource_info.limit_cpu,
                        ram_limit: resource_info.limit_memory,
                        healthy: resource_info.healthy,
                        cpu_usage,
                        ram_usage,
                        time: Some(chrono::Utc::now()),
                        reason: resource_info.reason,
                    };

                    tokio::spawn(async move {
                        let result: Result<usize, anyhow::Error> =
                            async_write_transaction(pool, move |connection| {
                                Ok(insert_into(pod_metrics)
                                    .values(&new_metric)
                                    .execute(connection)?)
                            })
                            .await;

                        match result {
                            Ok(_) => info!("Inserted pod metric for {}", pod_name),
                            Err(err) => error!("Failed to insert pod metric for {}: {}", pod_name, err),
                        }
                    });
                }
            })
            .await;
    }
}
