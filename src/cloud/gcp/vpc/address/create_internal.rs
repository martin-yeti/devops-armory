use std::time::Duration;
use super::models::IpAddress;

/// Create internal IP global address
/// Project ID, token, ip_name need to be provided
pub async fn create_internal_address(
    token: String,
    project: String,
    ip_name: String,
    ip_address: String,
    ip_prefix: String,
    network_tier: String,
    ip_version: String,
    ip_address_type: String,
    ip_address_purpose: String,
    network_name: String
) -> Result<(), std::io::Error> {

    let address_body: IpAddress = IpAddress { 
        name: ip_name, 
        address: ip_address, 
        prefixLength: ip_prefix,
        networkTier: network_tier, 
        ipVersion: ip_version, 
        addressType: ip_address_type, 
        purpose: ip_address_purpose, 
        network: network_name 
    };

    let client = awc::Client::default();
    let request = client.post(format!("https://compute.googleapis.com/compute/v1/projects/{project}/global/addresses"))
        .bearer_auth(&token)
        .insert_header(("Content-Type", "application/json"))
        .timeout(Duration::from_secs(30))
        .send_json(&address_body)
        .await
        .expect("Request POST new address could not been sent");

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

