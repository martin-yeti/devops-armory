use std::time::Duration;

use super::models::{
    UpdateGkeCluster,
};

/// Update GKE cluster
/// Token, project, location need to be provided
pub async fn update_gke_cluster(
    token: String,
    project: String,
    gke_cluster_location: String,
    gke_cluster_name: String,
    gke_cluster_update_data: UpdateGkeCluster
) -> Result<(), std::io::Error> {

    let update_cluster= gke_cluster_update_data;

    let client = awc::Client::default();
    let request = client.put(format!("https://container.googleapis.com/v1/projects/{project}/locations/{gke_cluster_location}/clusters/{gke_cluster_name}"))
        .bearer_auth(token)
        .insert_header(("Content-Type", "application/json"))
        .timeout(Duration::from_secs(30))
        .send_json(&update_cluster)
        .await
        .expect("Request: Update GKE cluster could not been sent");

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
            println!("Requested resource already exists: {:?}", respone);
        }
        _ => {
            println!("Request status mismatch. Check response: {:?}", respone);
        }
    }

    Ok(())

}
