use std::time::Duration;

use super::models::VpcSubnetPrivateIpGoogleAccess;

pub async fn set_private_access_for_subnet(
    token: String,
    project: String,
    subnet_region: String,
    subnet_name: String,
    subnet_private_access: bool
) -> Result<(), std::io::Error> {

    let vpc_subnet_body: VpcSubnetPrivateIpGoogleAccess = VpcSubnetPrivateIpGoogleAccess { 
        privateIpGoogleAccess: subnet_private_access 
    }; 

    let client = awc::Client::default();
    let request = client.post(format!("https://compute.googleapis.com/compute/v1/projects/{project}/regions/{subnet_region}/subnetworks/{subnet_name}/setPrivateIpGoogleAccess"))
        .bearer_auth(token)
        .insert_header(("Content-Type", "application/json"))
        .timeout(Duration::from_secs(30))
        .send_json(&vpc_subnet_body)
        .await
        .expect("Request: VPC private access could not been sent");

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
