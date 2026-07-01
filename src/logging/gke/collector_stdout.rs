use awc::*;
use actix_web::web::Bytes;
use openssl::ssl::{
    SslConnector,
    SslMethod,
    SslVerifyMode
};

use futures::stream::select_all;
use futures::{StreamExt, TryStreamExt};
use tuple_conv::*;

use crate::logging::alerts::models::Log;

/// Get logs from GKE pod
/// Token, cluster endpoint, namespace and pod name need to be provided
pub async fn gke_log_collector_stdout(
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
                    let gcp_id = gcp_id.as_str();
                    let gcp_region = gke_cluster_region.as_str();
                    let project_name = project_name.as_str();
                    let chunk_string =
                        std::str::from_utf8(&chunk_bytes).expect("Non-UTF8 bytes");
                    let iter = chunk_string.split_once(char::is_whitespace);
                    let vec = iter.unwrap_or_default().to_vec();
                    let log = Log {
                        time: vec.get(0).map(|x| x.to_string()).unwrap_or_default(),
                        message: vec.get(1).map(|x| x.to_string()).unwrap_or_default(),
                        host: pod_name.clone(),
                        google_project_id: gcp_id.to_string(),
                        region: gcp_region.to_string(),
                        project_id: project_name.to_string(),
                    };
                    println!("{:#?}", log);
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
