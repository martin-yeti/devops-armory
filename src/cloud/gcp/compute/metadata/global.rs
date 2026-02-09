use std::time::Duration;

use crate::cloud::gcp::compute::models::ComputeMetadata;

/// Set project metadata
/// Token, project name need to be provided
pub async fn set_project_meta(
    token: String,
    project_name: String,
    gcp_project_metadata: ComputeMetadata
) -> Result<(), std::io::Error> {

    let metadata = gcp_project_metadata;

    let client = awc::Client::default();
    let request = client.patch(format!("https://compute.googleapis.com/compute/v1/projects/{project_name}/regions/global"))
        .bearer_auth(token)
        .insert_header(("Content-Type", "application/json"))
        .timeout(Duration::from_secs(30))
        .send_json(&metadata)
        .await
        .expect("Request PATCH project metadata could not been sent");

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
