use awc::{
    Client, Connector,
};

use openssl::ssl::{
    SslConnector, 
    SslMethod, 
    SslVerifyMode
};

use super::models::NodeList;

/// Get the node info 
/// Token, gke endpoint and namespace need to be provided
pub async fn get_node_info(
    token: String,
    gke_cluster_endpoint: String,
    gke_cluster_node: String
) -> Result<(), std::io::Error> {

    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE);
    let myconnector = builder.build();
    let client = Client::builder()
        .connector(Connector::new().openssl(myconnector))
        .finish();

    let get_nodes_request = client
        .get(format!("https://{gke_cluster_endpoint}:443/api/v1/nodes/{gke_cluster_node}"))
        .bearer_auth(format!("{token}"))
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
