use std::time::Duration;

use awc::{
    Client, Connector,
};

use openssl::ssl::{
    SslConnector, 
    SslMethod, 
    SslVerifyMode
};

use crate::cloud::gcp::gke::gateway::models::GatewayGet;

/// Get GKE gateway info
/// Token, GKE endpoint, namespace and gateway name need to be provided
pub async fn get_gke_gateway(
    token: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    gke_cluster_gateway: String    
) -> Result<(), std::io::Error> {

    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE);
    let myconnector = builder.build();
    let client = Client::builder()
        .connector(Connector::new().openssl(myconnector))
        .finish();

    let get_gateway_request = client
        .get(format!("https://{gke_cluster_endpoint}:443/apis/gateway.networking.k8s.io/v1/namespaces/{gke_cluster_namespace}/gateways/{gke_cluster_gateway}"))
        .bearer_auth(format!("{token}"))
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .expect("Request GET gateway failed");

    let mut req = get_gateway_request;
    let req_status = req.status().as_u16();
    let response = req.body().await.unwrap_or_default();

    match req_status {
        200 => {
            println!("Request has been successfull: Status: {:?}, {:?}", req_status, response);
        },
        201 => {
            println!("Successfully modified gateway: {:?}", response);
        }
        400 => {
            println!("Bad Request. Check URL parameters or body: {:?}", response);
        },
        403 => {
            println!("You don't have access to perform such request: {:?}", response);
        }
        404 => {
            println!("Requested resource does not exists: {:?}", response);
        },
        409 => {
            println!("Requested resource already exists! {:?}", response)
        }
        _ => {
            println!("Request status mismatch. Check response: {:?}", response);
        }
    }

    Ok(())
    
}

/// Get GKE gateway resource version
/// Token, GKE endpoint, namespace and gateway name need to be provided
pub async fn get_gke_gateway_resource_version(
    token: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    gke_cluster_gateway: String    
) -> Result<String, std::io::Error> {

    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE);
    let myconnector = builder.build();
    let client = Client::builder()
        .connector(Connector::new().openssl(myconnector))
        .finish();

    let get_gateway_request = client
        .get(format!("https://{gke_cluster_endpoint}:443/apis/gateway.networking.k8s.io/v1/namespaces/{gke_cluster_namespace}/gateways/{gke_cluster_gateway}"))
        .bearer_auth(format!("{token}"))
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .expect("Request GET gateway failed")
        .json::<GatewayGet>()
        .await
        .unwrap_or_default();

    Ok(get_gateway_request.metadata.resourceVersion)
    
}
