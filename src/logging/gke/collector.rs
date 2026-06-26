use awc::*;
use openssl::ssl::{
    SslConnector, 
    SslMethod, 
    SslVerifyMode
};

use futures::TryStreamExt;

/// Get logs from GKE pod
/// Token, cluster endpoint, namespace and pod name need to be provided
pub async fn gke_log_collector(
    token: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    pod_name: String
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

    for x in streams.into_iter() {
        println!("{:?}", x)
    }

    Ok(())

}
