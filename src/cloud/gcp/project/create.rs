use std::time::Duration;

use super::models::GcpProjectCreate;

/// Create GCP project
/// Token and organization ID are required
pub async fn create_gcp_project(
    token: String,
    org_id: String,
    project_id: String,
    project_state: String,
    project_display_name: String
) -> Result<(), std::io::Error> {

    let project_body: GcpProjectCreate = GcpProjectCreate { 
        parent: org_id, 
        projectId: project_id, 
        state: project_state, 
        displayName: project_display_name
    }; 

    let client = awc::Client::default();
    let request = client.post(format!("https://cloudresourcemanager.googleapis.com/v3/projects"))
        .bearer_auth(token)
        .insert_header(("Content-Type", "application/json"))
        .timeout(Duration::from_secs(30))
        .send_json(&project_body)
        .await
        .expect("Request: Create Project could not been sent");

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
