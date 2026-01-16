use std::time::Duration;

use super::models::{
    VpcSubnetUpdate,
    SecondaryIpRanges
};

/// Update VPC Subnet
/// Project ID, token, need to be provided
pub async fn update_vpc_subnetwork(
    token: String,
    project: String,
    subnet_name: String,
    subnet_region: String,
    subnet_sec_ip_ranges: Vec<SecondaryIpRanges>,
    subnet_fingerprint: String,
) -> Result<(), std::io::Error> {

    let vpc_subnet_body: VpcSubnetUpdate = VpcSubnetUpdate { 
        secondaryIpRanges: subnet_sec_ip_ranges,
        fingerprint: subnet_fingerprint
    }; 

    let client = awc::Client::default();
    let request = client.patch(format!("https://compute.googleapis.com/compute/v1/projects/{project}/regions/{subnet_region}/subnetworks/{subnet_name}"))
        .bearer_auth(token)
        .insert_header(("Content-Type", "application/json"))
        .timeout(Duration::from_secs(30))
        .send_json(&vpc_subnet_body)
        .await
        .expect("Request: Update VPC subnet could not been sent");

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
