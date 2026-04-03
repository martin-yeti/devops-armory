use std::time::Duration;

use super::models::GetIpAddress;

/// Get regional external IP
/// Project ID, token, ip_name need to be provided
pub async fn get_regional_ip(
    token: String,
    project: String,
    region: String,
    ip_name: String
) -> Result<(), std::io::Error> {

    let client = awc::Client::default();
    let get_ip_request = client
        .get(format!("https://compute.googleapis.com/compute/v1/projects/{project}/regions/{region}/addresses/{ip_name}"))
        .bearer_auth(&token)
        .insert_header(("Content-Type", "application/json"))
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .expect("Request GET global IP failed");

    let mut req = get_ip_request;
    let req_status = req.status().as_u16();
    let respone = req.body().await.unwrap_or_default();

    match req_status {
        200 => {
            println!("Request has been successfull: Status: {:?}, {:?}", req_status, respone);
        },
        201 => {
            println!("Successfully created service: {:?}", respone);
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

/// Get regional external IP name
/// Project ID, token, ip_name need to be provided
pub async fn get_regional_ip_name(
    token: String,
    project: String,
    region: String,
    ip_name: String
) -> Result<String, std::io::Error> {

    let client = awc::Client::default();
    let get_ip_request = client
        .get(format!("https://compute.googleapis.com/compute/v1/projects/{project}/regions/{region}/addresses/{ip_name}"))
        .bearer_auth(&token)
        .insert_header(("Content-Type", "application/json"))
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .expect("Request GET global IP failed")
        .json::<GetIpAddress>()
        .await
        .unwrap_or_default();

    Ok(get_ip_request.name)

}
