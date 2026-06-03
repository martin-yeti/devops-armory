use std::time::Duration;

use awc::{
    Client, Connector,
};

use openssl::ssl::{
    SslConnector, 
    SslMethod, 
    SslVerifyMode
};

use super::models::{
    NodeList,
    NodePool
};

/// Get the node list from cluster
/// Token, gke endpoint and namespace need to be provided
pub async fn node_list(
    token: String,
    gke_cluster_endpoint: String,
) -> Result<(), std::io::Error> {

    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE);
    let myconnector = builder.build();
    let client = Client::builder()
        .connector(Connector::new().openssl(myconnector))
        .finish();

    let get_nodes_request = client
        .get(format!("https://{gke_cluster_endpoint}:443/api/v1/nodes"))
        .bearer_auth(format!("{token}"))
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .expect("Request get nodes failed");

    let  mut req = get_nodes_request;

    let req_status = req.status().as_u16();

    let response = req.json::<NodeList>().await.unwrap_or_default();

    match req_status {
        200 => {
            println!("Request has been successfull: Status: {:?}, {:?}", req_status, response);
        },
        201 => {
            println!("Successfully created ingress: {:?}", response);
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


/// Get the node pool list from cluster via container api
/// Token, gke endpoint and namespace need to be provided
pub async fn node_pool_list(
    token: String,
    gcp_project_id: String,
    gke_zone_location: String,
    gke_cluster_name: String
) -> Result<(), std::io::Error> {

    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE);
    let myconnector = builder.build();
    let client = Client::builder()
        .connector(Connector::new().openssl(myconnector))
        .finish();

    let get_nodepools_request = client
        .get(format!("https://container.googleapis.com/v1/projects/{gcp_project_id}/locations/{gke_zone_location}/clusters/{gke_cluster_name}/nodePools"))
        .bearer_auth(format!("{token}"))
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .expect("Request get nodes failed");
    
    let  mut req = get_nodepools_request;

    let req_status = req.status().as_u16();

    let response = req.json::<NodePool>().await.unwrap_or_default();

    match req_status {
        200 => {
            println!("Request has been successfull: Status: {:?}, {:?}", req_status, response);
        },
        201 => {
            println!("Successfully created ingress: {:?}", response);
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
