use std::time::Duration;

use super::models::CreateSSL;

/// Create GCP SSL cert
/// Token and project name need to be provided
pub async fn create_ssl(
    token: String,
    project: String,
    gcp_ssl_body: CreateSSL
) -> Result<(), std::io::Error> {

    let data = gcp_ssl_body;

    let client = awc::Client::default();
    let create_ssl_request = client.post(format!("https://compute.googleapis.com/compute/v1/projects/{project}/global/sslCertificates"))
        .bearer_auth(&token)
        .insert_header(("Content-Type", "application/json"))
        .timeout(Duration::from_secs(30))
        .send_json(&data)
        .await
        .expect("Request CREATE global SSL failed");

    let mut req = create_ssl_request;
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

