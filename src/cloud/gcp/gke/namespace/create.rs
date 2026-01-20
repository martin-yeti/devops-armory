use std::time::Duration;

use awc::{
    Client, Connector,
};

use openssl::ssl::{
    SslConnector, 
    SslMethod, 
    SslVerifyMode
};

use super::models::CreateNamespace;

/// Create namespace for existing cluster
/// Token, GKE cluster endpoint are required
pub async fn create_namespace(
    token: String,
    gke_cluster_endpoint: String,
    gke_namespace_body: CreateNamespace
) -> Result<(), std::io::Error> {

    let namespace_body = gke_namespace_body;

    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE);
    let myconnector = builder.build();
    let client = Client::builder()
        .connector(Connector::new().openssl(myconnector))
        .finish();

    let create_namespace_request = client
        .post(format!("https://{gke_cluster_endpoint}:443/api/v1/namespaces"))
        .bearer_auth(format!("{token}"))
        .timeout(Duration::from_secs(30))
        .send_json(&namespace_body)
        .await
        .expect("Request: CREATE namespace failed");

    let mut req = create_namespace_request;
    let req_status = req.status().as_u16();
    let respone = req.body().await.unwrap_or_default();

    match req_status {
        200 => {
            println!("Request has been successfull: Status: {:?}, {:?}", req_status, respone);
        },
        201 => {
            println!("Successfully created namespace: {:?}", respone);
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
