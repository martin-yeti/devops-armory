use std::time::Duration;
use super::models::{
    VpcRouter, 
    RouterNats, 
};

/// Create VPC router and NAT
/// Project ID, token, ip_name need to be provided
pub async fn create_router(
    token: String,
    project: String,
    router_name: String,
    router_nats: Vec<RouterNats>,
    network_name: String,
    vpc_region: String
) -> Result<(), std::io::Error> {

    let router_body: VpcRouter = VpcRouter { 
        name: router_name, 
        nats: router_nats,
        network: network_name, 
        region: vpc_region.clone()
    };

    let client = awc::Client::default();
    let request = client.post(format!("https://compute.googleapis.com/compute/v1/projects/{project}/regions/{vpc_region}/routers"))
        .bearer_auth(token)
        .insert_header(("Content-Type", "application/json"))
        .timeout(Duration::from_secs(30))
        .send_json(&router_body)
        .await
        .expect("Request: Create Router could not been sent");

    let mut req = request;
    let req_status = req.status().as_u16();
    let respone = req.body().await.unwrap_or_default();

    match req_status {
        200 => {
            println!("Request has been successfull: Status: {:?}, {:?}", req_status, respone);
        },
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

