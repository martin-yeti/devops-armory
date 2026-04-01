use std::time::Duration;

use awc::{
    Client, Connector,
};

use openssl::ssl::{
    SslConnector, 
    SslMethod, 
    SslVerifyMode
};

use super::models::HTTPRouteGetParent;

/// GET GKE route
/// Token, GKE endpoint and namespace need to be provided
pub async fn get_gke_route(
    token: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    gke_cluster_route: String
) -> Result<(), std::io::Error> {

    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE);
    let myconnector = builder.build();
    let client = Client::builder()
        .connector(Connector::new().openssl(myconnector))
        .finish();

    let get_route_request = client
        .get(format!("https://{gke_cluster_endpoint}:443/apis/gateway.networking.k8s.io/v1/namespaces/{gke_cluster_namespace}/httproutes/{gke_cluster_route}"))
        .bearer_auth(format!("{token}"))
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .expect("Request GET route failed");

    let mut req = get_route_request;
    let req_status = req.status().as_u16();
    let response = req.body().await.unwrap_or_default();

    match req_status {
        200 => {
            println!("Request has been successfull: Status: {:?}, {:?}", req_status, response);
        },
        201 => {
            println!("Successfully deleted route: {:?}", response);
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


/// GET GKE route resource version
/// Token, GKE endpoint, route name and namespace need to be provided
pub async fn get_gke_route_resource_version(
    token: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    gke_cluster_route: String
) -> Result<String, std::io::Error> {

    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE);
    let myconnector = builder.build();
    let client = Client::builder()
        .connector(Connector::new().openssl(myconnector))
        .finish();

    let get_route_request = client
        .get(format!("https://{gke_cluster_endpoint}:443/apis/gateway.networking.k8s.io/v1/namespaces/{gke_cluster_namespace}/httproutes/{gke_cluster_route}"))
        .bearer_auth(format!("{token}"))
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .expect("Request GET route failed")
        .json::<HTTPRouteGetParent>()
        .await
        .unwrap_or_default();

    Ok(get_route_request.items[0].resourceVersion.clone())

}