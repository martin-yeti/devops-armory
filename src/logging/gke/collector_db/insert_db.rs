use awc::*;
use actix_web::web::Bytes;
use diesel::insert_into;
use diesel::RunQueryDsl;
use openssl::ssl::{
    SslConnector,
    SslMethod,
    SslVerifyMode
};

use futures::stream::SelectAll;
use futures::{StreamExt, TryStreamExt};

use super::connection::establish_connection;
use super::models::NewLog;
use crate::cloud::gcp::gke::pod::list::pod_list_vector;
use crate::logging::gke::collector_db::schema::logs::dsl::logs;

/// Get logs from GKE pod
/// Token, cluster endpoint, namespace and pod name need to be provided
pub async fn gke_log_collector_db(
    token: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    gke_pod_list: &Vec<String>,
    gke_pod_phrase: &Vec<String>,
    project_name: String,
    project_region: String,
    gcp_id: String,
    gke_cluster_region: String,
) -> Result<(),std::io::Error> {
    
    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE);

    let myconnector = builder.build();

    let client = Client::builder()
        .connector(Connector::new().openssl(myconnector))
        .finish();

    let filtered_k8s_hostname = gke_pod_phrase;
    let mut line_buffers: std::collections::HashMap<String, Vec<u8>> = std::collections::HashMap::new();

    type PodStream = std::pin::Pin<Box<dyn futures::Stream<Item = Result<(String, Bytes), awc::error::PayloadError>>>>;

    // Streams for pods we're currently attached to, and the set of pod
    // names they cover. A stream is dropped from `streams` automatically
    // once it ends (e.g. its pod is deleted), but `active_pods` is only
    // reconciled against a fresh pod listing below.
    let mut streams: SelectAll<PodStream> = SelectAll::new();
    let mut active_pods: std::collections::HashSet<String> = std::collections::HashSet::new();

    // Periodically re-list pods and attach any new matches. This runs
    // independently of whether currently-attached streams have ended, so a
    // deleted pod's replacement gets picked up even if the old stream
    // never cleanly signals completion.
    let mut pod_refresh = tokio::time::interval(std::time::Duration::from_secs(5));

    loop {
        tokio::select! {
            maybe_chunk = streams.next(), if !streams.is_empty() => {
                match maybe_chunk {
                    Some(Ok((pod_name, chunk_bytes))) if chunk_bytes.is_empty() => {
                        // End-of-stream marker (see below): the underlying
                        // log stream closed, for any reason (pod deleted,
                        // container restarted, connection reset). Drop our
                        // bookkeeping so the next pod refresh can reattach
                        // if the pod is still around.
                        eprintln!("Log stream for pod {} ended; will retry on next refresh", pod_name);
                        active_pods.remove(&pod_name);
                        line_buffers.remove(&pod_name);
                    }
                    Some(Ok((pod_name, chunk_bytes))) => {
                        let buffer = line_buffers.entry(pod_name.clone()).or_default();
                        buffer.extend_from_slice(&chunk_bytes);

                        // Chunk boundaries are arbitrary byte offsets, not
                        // character-aligned, so buffer raw bytes and only
                        // decode once we know a line is fully terminated.
                        // Any trailing partial line (and any partial
                        // multi-byte character within it) stays buffered
                        // for the next chunk.
                        if let Some(last_newline) = buffer.iter().rposition(|&b| b == b'\n') {
                            let remainder = buffer.split_off(last_newline + 1);
                            let complete_bytes = std::mem::replace(buffer, remainder);
                            let text = String::from_utf8_lossy(&complete_bytes);

                            let connection = &mut establish_connection();
                            for line in text.split('\n') {
                                let line = line.trim_end_matches('\r');
                                if line.is_empty() {
                                    continue;
                                }
                                let (time_str, message) = line
                                    .split_once(char::is_whitespace)
                                    .unwrap_or(("", line));
                                let time = time_str
                                    .parse::<chrono::DateTime<chrono::Utc>>()
                                    .ok();
                                let new_log = NewLog {
                                    time,
                                    message: message.to_string(),
                                    host: pod_name.clone(),
                                    google_project_id: gcp_id.clone(),
                                    region: gke_cluster_region.clone(),
                                    project_id: project_name.clone(),
                                };
                                insert_into(logs)
                                    .values(&new_log)
                                    .on_conflict_do_nothing()
                                    .execute(connection)
                                    .unwrap_or_default();
                            }
                        }
                    }
                    Some(Err(err)) => {
                        eprintln!("Failed to read stream chunk: {}", err);
                    }
                    None => {}
                }
            }
            _ = pod_refresh.tick() => {
                let (current_pods, listing_succeeded): (Vec<String>, bool) = match pod_list_vector(
                    token.clone(),
                    gke_cluster_endpoint.clone(),
                    gke_cluster_namespace.clone(),
                )
                .await
                {
                    Ok(pods) => (pods.into_iter().map(|item| item.metadata.name).collect(), true),
                    Err(err) => {
                        eprintln!("Failed to list pods, falling back to initial pod list: {}", err);
                        (gke_pod_list.clone(), false)
                    }
                };

                for pod_name in &current_pods {
                    if active_pods.contains(pod_name) {
                        continue;
                    }
                    if !filtered_k8s_hostname.iter().any(|p| pod_name.contains(p)) {
                        continue;
                    }

                    let pod_name_owned = pod_name.clone();
                    let pod_name_for_end = pod_name.clone();
                    let sent = client
                        .get(&format!("https://{gke_cluster_endpoint}/api/v1/namespaces/{gke_cluster_namespace}/pods/{pod_name}/log?&tailLines=10&follow&timestamps=true"))
                        .bearer_auth(&token.clone())
                        .insert_header(("Content-Type", "application/json"))
                        .send()
                        .await;

                    match sent {
                        Ok(response) if !response.status().is_success() => {
                            // Most commonly the pod is still starting up
                            // (e.g. ContainerCreating) and the API server
                            // rejects the log request. Don't mark it as
                            // active so we retry on the next refresh.
                            eprintln!(
                                "Log endpoint for pod {} returned status {}; will retry on next refresh",
                                pod_name,
                                response.status()
                            );
                        }
                        Ok(response) => {
                            let res = response
                                .into_stream()
                                .map(move |chunk| chunk.map(|b| (pod_name_owned.clone(), b)))
                                // Signal completion with an empty chunk so the
                                // consumer can clear active_pods and allow a
                                // retry, since SelectAll otherwise drops ended
                                // streams silently.
                                .chain(futures::stream::once(async move {
                                    Ok((pod_name_for_end, Bytes::new()))
                                }));
                            streams.push(Box::pin(res));
                            active_pods.insert(pod_name.clone());
                            println!("Attached to log stream for pod {}", pod_name);
                        }
                        Err(err) => {
                            eprintln!("Failed to connect to stream for pod {}: {}", pod_name, err);
                        }
                    }
                }

                // Only reconcile against an authoritative listing; the
                // static fallback list used on a listing error isn't a
                // complete picture of what's currently running.
                if listing_succeeded {
                    active_pods.retain(|pod| current_pods.contains(pod));
                }

                if streams.is_empty() {
                    println!("No matching pods to stream from. Retrying in 5s");
                }
            }
        }
    }
}
