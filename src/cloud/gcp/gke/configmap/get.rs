use std::time::Duration;

use awc::{
    Client, Connector,
};

use openssl::ssl::{
    SslConnector, 
    SslMethod, 
    SslVerifyMode
};

/// Get GKE ConfigMap
/// Token, GKE endpoint, namespace need to be provided
pub async fn get_gke_configmap(
    token: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    gke_configmap_name: String
) -> Result<(), std::io::Error> {

    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE);
    let myconnector = builder.build();
    let client = Client::builder()
        .connector(Connector::new().openssl(myconnector))
        .finish();

    let get_configmap_request = client
        .get(format!("https://{gke_cluster_endpoint}:443/api/v1/namespaces/{gke_cluster_namespace}/configmaps/{gke_configmap_name}"))
        .bearer_auth(format!("{token}"))
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .expect("Failed to GET configmap in current namespace");

    let mut req = get_configmap_request;
    let req_status = req.status().as_u16();
    let respone = req.body().await.unwrap_or_default();

    match req_status {
        200 => {
            println!("Request has been successfull: Status: {:?}, {:?}", req_status, respone);
        },
        201 => {
            println!("Successfully created configmap Config: {:?}", respone);
        }
        400 => {
            println!("Bad Request. Check URL parameters or body: {:?}", respone);
        },
        403 => {
            println!("You don't have access to perform such request: {:?}", respone);
        }
        404 => {
            println!("Requested resource does not exists: {:?}", respone);
        },
        409 => {
            println!("Requested resource already exists! {:?}", respone)
        }
        _ => {
            println!("Request status mismatch. Check response: {:?}", respone);
        }
    }

    Ok(())
    
}
