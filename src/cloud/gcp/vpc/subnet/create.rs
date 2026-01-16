use std::time::Duration;

use super::models::VpcSubnet;

/// Create VPN subnetwork
/// Project ID, token, need to be provided
pub async fn create_vpc_subnetwork(
    token: String,
    project: String,
    subnet_region: String,
    subnet_name: String,
    subnet_desc: String,
    subnet_network: String,
    subnet_ipcidrrange: String
) -> Result<(), std::io::Error> {

    let vpc_subnet_body: VpcSubnet = VpcSubnet { 
        name: subnet_name,
        description: subnet_desc,
        network: subnet_network,
        ipCidrRange: subnet_ipcidrrange,
        region: subnet_region.clone()
    }; 

    let client = awc::Client::default();
    let request = client.post(format!("https://compute.googleapis.com/compute/v1/projects/{project}/regions/{subnet_region}/subnetworks"))
        .bearer_auth(token)
        .insert_header(("Content-Type", "application/json"))
        .timeout(Duration::from_secs(30))
        .send_json(&vpc_subnet_body)
        .await
        .expect("Request: Create VPC subnet could not been sent");

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
