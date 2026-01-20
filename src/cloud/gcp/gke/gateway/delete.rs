use std::time::Duration;

use awc::{
    Client, Connector,
};

use openssl::ssl::{
    SslConnector, 
    SslMethod, 
    SslVerifyMode
};

/// Delete GKE gateway
/// Token, GKE endpoint, namespace and gateway name need to be provided
pub async fn delete_gke_gateway(
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

    let delete_gateway_request = client
        .delete(format!("https://{gke_cluster_endpoint}:443/apis/gateway.networking.k8s.io/v1/namespaces/{gke_cluster_namespace}/gateways/{gke_cluster_gateway}"))
        .bearer_auth(format!("{token}"))
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .expect("Request DELETE gateway failed");

    let mut req = delete_gateway_request;
    let req_status = req.status().as_u16();
    let respone = req.body().await.unwrap_or_default();

    match req_status {
        200 => {
            println!("Request has been successfull: Status: {:?}, {:?}", req_status, respone);
        },
        201 => {
            println!("Successfully deleted gateway: {:?}", respone);
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
