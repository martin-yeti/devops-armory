use std::time::Duration;

use awc::{
    Client, Connector,
};

use openssl::ssl::{
    SslConnector, 
    SslMethod, 
    SslVerifyMode
};

use super::models::Gateway;

/// Update GKE gateway
/// Token, GKE endpoint and namespace need to be provided
pub async fn update_gke_gateway(
    token: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    gke_cluster_gateway_name: String,
    gke_cluster_gateway: Gateway
) -> Result<(), std::io::Error> {

    let gateway_request_body = gke_cluster_gateway;

    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE);
    let myconnector = builder.build();
    let client = Client::builder()
        .connector(Connector::new().openssl(myconnector))
        .finish();
    // V1
    let update_gateway_request = client
        .put(format!("https://{gke_cluster_endpoint}:443/apis/gateway.networking.k8s.io/v1/namespaces/{gke_cluster_namespace}/gateways/{gke_cluster_gateway_name}"))
        .bearer_auth(format!("{token}"))
        .timeout(Duration::from_secs(30))
        .send_json(&gateway_request_body)
        .await
        .expect("Request CREATE gateway failed");

    let mut req = update_gateway_request;
    let req_status = req.status().as_u16();
    let response = req.body().await.unwrap_or_default();

    match req_status {
        200 => {
            println!("Request has been successfull: Status: {:?}, {:?}", req_status, response);
        },
        201 => {
            println!("Successfully created gateway: {:?}", response);
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

