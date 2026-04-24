use std::time::Duration;

use crate::cloud::gcp::compute::models::GetVMInstanceInfo;

/// Get GCP instance info
/// Token, project name need to be provided
pub async fn get_gcp_vm_info(
    token: String,
    project_name: String,
    zone_name: String,
    instance_name: String
) -> Result<GetVMInstanceInfo, std::io::Error> {

    let client = awc::Client::default();
    let request = client
        .get(format!("https://compute.googleapis.com/compute/v1/projects/{project_name}/zones/{zone_name}/instances/{instance_name}"))
        .bearer_auth(token)
        .insert_header(("Content-Type", "application/json"))
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .expect("Request GET instance info could not been sent")
        .json::<GetVMInstanceInfo>()
        .await
        .unwrap_or_default();

    Ok(request)

}

// Debug - raw bytes
pub async fn get_gcp_vm_info_raw(
    token: String,
    project_name: String,
    zone_name: String,
    instance_name: String
) -> Result<(), std::io::Error> {

    let client = awc::Client::default();
    let request = client
        .get(format!("https://compute.googleapis.com/compute/v1/projects/{project_name}/zones/{zone_name}/instances/{instance_name}"))
        .bearer_auth(token)
        .insert_header(("Content-Type", "application/json"))
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .expect("Request GET instance info could not been sent");

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
