use std::time::Duration;

use super::models::{
    CreateGkeCluster,
    GkeCluster,
};

/// Create GKE cluster
/// Token, project and location needs to be provided
pub async fn create_gke_cluster(
    token: String,
    project: String,
    location: String,
    gke_cluster: GkeCluster,
) -> Result<(), std::io::Error> {

    let cluster_body: CreateGkeCluster = CreateGkeCluster { 
        cluster: gke_cluster
    }; 

    let client = awc::Client::default();
    let request = client.post(format!("https://container.googleapis.com/v1/projects/{project}/locations/{location}/clusters"))
        .bearer_auth(token)
        .insert_header(("Content-Type", "application/json"))
        .timeout(Duration::from_secs(30))
        .send_json(&cluster_body)
        .await
        .expect("Request: Create GKE cluster could not been sent");

    let mut req = request;
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

