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

    let pod_list = gke_pod_list;
    let filtered_k8s_hostname = gke_pod_phrase;

    let mut streams: Vec<std::pin::Pin<Box<dyn futures::Stream<Item = Result<(String, Bytes), awc::error::PayloadError>>>>> = Vec::new();

    for pod_name in pod_list {
        if filtered_k8s_hostname.iter().any(|p| pod_name.contains(p)) {
            let pod_name_owned = pod_name.clone();
            let res = client
                .get(&format!("https://{gke_cluster_endpoint}/api/v1/namespaces/{gke_cluster_namespace}/pods/{pod_name}/log?&tailLines=10&follow&timestamps=true"))
                .bearer_auth(&token.clone())
                .insert_header(("Content-Type", "application/json"))
                .send()
                .await
                .expect("Fail to connect to stream")
                .into_stream()
                .map(move |chunk| chunk.map(|b| (pod_name_owned.clone(), b)));

            streams.push(Box::pin(res));
        }
    }

    let mut combined_stream = select_all(streams);

    loop {
        match combined_stream.next().await {
            Some(chunk) => match chunk {
                Ok((pod_name, chunk_bytes)) => {
                    let chunk_string =
                        std::str::from_utf8(&chunk_bytes).expect("Non-UTF8 bytes");
                    let (time_str, message) = chunk_string
                        .split_once(char::is_whitespace)
                        .unwrap_or(("", chunk_string));
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
                    let connection = &mut establish_connection();
                    insert_into(logs)
                        .values(&new_log)
                        .on_conflict_do_nothing()
                        .execute(connection)
                        .unwrap_or_default();
                }
                Err(err) => {
                    eprintln!("Failed to read stream chunk: {}", err)
                }
            },
            None => {
                println!("Nothing to stream. Exiting");
                break;
            }
        }
    }

    Ok(())

}
