use awc::*;
use actix_web::web::Bytes;
use diesel::insert_into;
use diesel::RunQueryDsl;
use openssl::ssl::{
    SslConnector,
    SslMethod,
    SslVerifyMode
};

use futures::stream::select_all;
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
    let mut line_buffers: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    // Outer loop: whenever every stream we're following has ended (e.g. their
    // pods got deleted), re-list the current pods and re-attach instead of
    // exiting the collector entirely.
    loop {
        let current_pods = match pod_list_vector(
            token.clone(),
            gke_cluster_endpoint.clone(),
            gke_cluster_namespace.clone(),
        )
        .await
        {
            Ok(pods) => pods.into_iter().map(|item| item.metadata.name).collect(),
            Err(err) => {
                eprintln!("Failed to list pods, falling back to initial pod list: {}", err);
                gke_pod_list.clone()
            }
        };

        let mut streams: Vec<std::pin::Pin<Box<dyn futures::Stream<Item = Result<(String, Bytes), awc::error::PayloadError>>>>> = Vec::new();

        for pod_name in &current_pods {
            if filtered_k8s_hostname.iter().any(|p| pod_name.contains(p)) {
                let pod_name_owned = pod_name.clone();
                let sent = client
                    .get(&format!("https://{gke_cluster_endpoint}/api/v1/namespaces/{gke_cluster_namespace}/pods/{pod_name}/log?&tailLines=10&follow&timestamps=true"))
                    .bearer_auth(&token.clone())
                    .insert_header(("Content-Type", "application/json"))
                    .send()
                    .await;

                match sent {
                    Ok(response) => {
                        let res = response
                            .into_stream()
                            .map(move |chunk| chunk.map(|b| (pod_name_owned.clone(), b)));
                        streams.push(Box::pin(res));
                    }
                    Err(err) => {
                        eprintln!("Failed to connect to stream for pod {}: {}", pod_name, err);
                    }
                }
            }
        }

        if streams.is_empty() {
            println!("No matching pods to stream from. Retrying in 5s");
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            continue;
        }

        let mut combined_stream = select_all(streams);

        loop {
            match combined_stream.next().await {
                Some(chunk) => match chunk {
                    Ok((pod_name, chunk_bytes)) => {
                        let chunk_string =
                            std::str::from_utf8(&chunk_bytes).expect("Non-UTF8 bytes");
                        let buffer = line_buffers.entry(pod_name.clone()).or_default();
                        buffer.push_str(chunk_string);

                        // Only fully-terminated lines represent complete log entries;
                        // keep any trailing partial line buffered for the next chunk.
                        let mut lines: Vec<String> = buffer.split('\n').map(String::from).collect();
                        let remainder = lines.pop().unwrap_or_default();
                        *buffer = remainder;

                        let connection = &mut establish_connection();
                        for line in lines {
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
                    Err(err) => {
                        eprintln!("Failed to read stream chunk: {}", err)
                    }
                },
                None => {
                    println!("All pod streams ended. Refreshing pod list");
                    break;
                }
            }
        }
    }
}
