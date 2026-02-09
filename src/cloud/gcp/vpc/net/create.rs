use std::time::Duration;
use super::models::VpcNetwork;

/// Create VPC network
/// Project ID, token, ip_name need to be provided
pub async fn create_vpc_network(
    token: String,
    project: String,
    vpc_name: String,
    vpc_description: String,
    vpc_auto_create_subnet: bool
) -> Result<(), std::io::Error> {

    let vpc_body: VpcNetwork = VpcNetwork { 
        name: vpc_name,
        description: vpc_description,
        autoCreateSubnetworks: vpc_auto_create_subnet
    }; 

    let client = awc::Client::default();
    let request = client.post(format!("https://compute.googleapis.com/compute/v1/projects/{project}/global/networks"))
        .bearer_auth(token)
        .insert_header(("Content-Type", "application/json"))
        .timeout(Duration::from_secs(30))
        .send_json(&vpc_body)
        .await
        .expect("Request: Create VPC network could not been sent");

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


