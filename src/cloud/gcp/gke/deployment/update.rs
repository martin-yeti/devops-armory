use std::time::Duration;

use awc::{
    Client, Connector,
};

use openssl::ssl::{
    SslConnector, 
    SslMethod, 
    SslVerifyMode
};

use super::models::CreateDeployment;

/// Update GKE deployment
/// Token, endpoint, namespace, deployment name
pub async fn put_update_deployment(
    token: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    gke_cluster_deployment_name: String,
    gke_deployment_body: CreateDeployment
) -> Result<(), std::io::Error> {

    let update_deployment_body = gke_deployment_body;

    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE);
    let myconnector = builder.build();
    let client = Client::builder()
        .connector(Connector::new().openssl(myconnector))
        .finish();

    let update_deployment_request = client
        .put(format!("https://{gke_cluster_endpoint}:443/apis/apps/v1/namespaces/{gke_cluster_namespace}/deployments/{gke_cluster_deployment_name}"))
        .bearer_auth(format!("{token}"))
        .timeout(Duration::from_secs(30))
        .send_json(&update_deployment_body)
        .await
        .expect("Failed to update deployment in the current namespace");

    let mut req = update_deployment_request;
    let req_status = req.status().as_u16();
    let respone = req.body().await.unwrap_or_default();
    match req_status {
        200 => {
            println!("Request has been successfull: Status: {:?}, {:?}", req_status, respone);
        },
        201 => {
            println!("Successfully created deployment: {:?}", respone);
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
