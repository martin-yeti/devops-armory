use std::time::Duration;

use super::models::CreateVM;

/// Create GCP VM instance 
/// External and internal IP available via Option<VMAccessConfig>
/// Token, project name, net and subnet name and tags need to be provided
pub async fn create_gcp_vm(
    token: String,
    project_name: String,
    gcp_vm_zone: String,
    gcp_vm_specs: CreateVM
) -> Result<(), std::io::Error> {

    let gcp_vm_data = gcp_vm_specs;

    let client = awc::Client::default();
    let request = client.post(format!("https://compute.googleapis.com/compute/v1/projects/{project_name}/zones/{gcp_vm_zone}/instances"))
        .bearer_auth(&token)
        .insert_header(("Content-Type", "application/json"))
        .timeout(Duration::from_secs(30))
        .send_json(&gcp_vm_data)
        .await
        .expect("Request CREATE instance could not been sent");

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
