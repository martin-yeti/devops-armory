use std::time::Duration;

use super::models::GetSSL;

/// Get GCP global SSL cert
/// Token and project name need to be provided
pub async fn get_global_ssl(
    token: String,
    project: String,
    gcp_ssl_name: String
) -> Result<(), std::io::Error> {

    let client = awc::Client::default();
    let get_ssl_request = client
        .get(format!("https://compute.googleapis.com/compute/v1/projects/{project}/global/sslCertificates/{gcp_ssl_name}"))
        .bearer_auth(&token)
        .insert_header(("Content-Type", "application/json"))
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .expect("Request GET global SSL failed");

    let mut req = get_ssl_request;
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

/// Get GCP global SSL cert name
/// Token and project name need to be provided
pub async fn get_global_ssl_name(
    token: String,
    project: String,
    gcp_ssl_name: String
) -> Result<String, std::io::Error> {

    let client = awc::Client::default();
    let get_ssl_request = client
        .get(format!("https://compute.googleapis.com/compute/v1/projects/{project}/global/sslCertificates/{gcp_ssl_name}"))
        .bearer_auth(&token)
        .insert_header(("Content-Type", "application/json"))
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .expect("Request GET global SSL name failed")
        .json::<GetSSL>()
        .await
        .unwrap_or_default();

    Ok(get_ssl_request.name)

}
