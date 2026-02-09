use std::time::Duration;

use awc::{
    Client, Connector,
};

use openssl::ssl::{
    SslConnector, 
    SslMethod, 
    SslVerifyMode
};

use super::models::GkeSslCert;

/// Create GKE SSL cert
/// Token, GKE endpoint and namespace need to be provided
pub async fn create_gke_ssl(
    token: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    gke_ssl: GkeSslCert
) -> Result<(), std::io::Error> {

    let ssl_gke = gke_ssl;

    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE);
    let myconnector = builder.build();
    let client = Client::builder()
        .connector(Connector::new().openssl(myconnector))
        .finish();

    let create_gke_ssl_request = client
        .post(format!("https://{gke_cluster_endpoint}:443/apis/networking.gke.io/v1/namespaces/{gke_cluster_namespace}/managedcertificates"))
        .bearer_auth(format!("{token}"))
        .timeout(Duration::from_secs(30))
        .send_json(&ssl_gke)
        .await
        .expect("Failed to get pods in current namespace");

    let mut req = create_gke_ssl_request;
    let req_status = req.status().as_u16();
    let respone = req.body().await.unwrap_or_default();

    match req_status {
        200 => {
            println!("Request has been successfull: Status: {:?}, {:?}", req_status, respone);
        },
        201 => {
            println!("Successfully created Frontend Config: {:?}", respone);
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
