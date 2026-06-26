use awc::*;
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
pub async fn gke_log_collector(
    token: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    pod_name: String,
    filtered_pod_name: String,
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

    let mut streams = Vec::new();

    let res = client
        .get(&format!("https://{gke_cluster_endpoint}/api/v1/namespaces/{gke_cluster_namespace}/pods/{pod_name}/log?&tailLines=10&follow&timestamps=true"))   // <- Create request builder
        .bearer_auth(&token.clone())
        .insert_header(("Content-Type", "application/json"))
        .send()
        .await
        .expect("Fail to connect to stream")
        .into_stream();
    
    streams.push(res);

    let mut combined_stream = select_all(streams);

    loop {
        match combined_stream.next().await {
            Some(chunk) => match chunk {
                Ok(chunk_bytes) => {

                    let mut host_name = "".to_string();
                    if pod_name.contains(&filtered_pod_name) {
                        let gcp_id = project_name.as_str();
                        let gcp_region = gke_cluster_region.as_str();
                        let project_name = project_name.as_str();
                        host_name = pod_name.to_owned();
                        let chunk_string =
                            std::str::from_utf8(&chunk_bytes).expect("Non-UTF8 bytes");
                        let iter = chunk_string.split_once(char::is_whitespace);
                        let vec = iter.unwrap_or_default().to_vec();
                        let log = Log {
                            time: vec.get(0).map(|x| x.to_string()).unwrap_or_default(),
                            message: vec.get(1).map(|x| x.to_string()).unwrap_or_default(),
                            host: host_name.to_string(),
                            google_project_id: gcp_id.to_string(),
                            region: gcp_region.to_string(),
                            project_id: project_name.to_string(),
                        };
                        println!("{:#?}", log);
                    }
                    
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
