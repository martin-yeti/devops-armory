use std::time::Duration;

use awc::{
    Client, Connector,
};

use openssl::ssl::{
    SslConnector, 
    SslMethod, 
    SslVerifyMode
};

use super::models::UpdateHealthCheckPolicy;

/// Update GKE healthcheck
/// Token, GKE endpoint, namespace need to be provided
pub async fn update_gke_healthcheck(
    token: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    gke_cluster_healtcheck_name: String,
    gke_cluster_healthcheck_config: UpdateHealthCheckPolicy
) -> Result<(), std::io::Error> {

    let healthcheck_request_body = gke_cluster_healthcheck_config;

    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE);
    let myconnector = builder.build();
    let client = Client::builder()
        .connector(Connector::new().openssl(myconnector))
        .finish();

    let update_healthcheck_request = client
        .patch(format!("https://{gke_cluster_endpoint}:443/apis/networking.gke.io/v1/namespaces/{gke_cluster_namespace}/healthcheckpolicies/{gke_cluster_healtcheck_name}"))
        .bearer_auth(format!("{token}"))
        .timeout(Duration::from_secs(30))
        .send_json(&healthcheck_request_body)
        .await
        .expect("Failed to update halthcheck config in current namespace");

    let mut req = update_healthcheck_request;
    let req_status = req.status().as_u16();
    let respone = req.body().await.unwrap_or_default();

    match req_status {
        200 => {
            println!("Request has been successfull: Status: {:?}, {:?}", req_status, respone);
        },
        201 => {
            println!("Successfully updated halthcheck Config: {:?}", respone);
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
