use std::time::Duration;

/// Delete GKE cluster
/// Token, project and location need to be provided
pub async fn delete_gke_cluster(
    token: String,
    project: String,
    gke_cluster_zone: String,
    gke_cluster_name: String
) -> Result<(), std::io::Error> {

    let client = awc::Client::default();
    let request = client.delete(format!("https://container.googleapis.com/v1/projects/{project}/locations/{gke_cluster_zone}/clusters/{gke_cluster_name}"))
        .bearer_auth(token)
        .insert_header(("Content-Type", "application/json"))
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .expect("Request DELETE cluster could not been sent");

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
