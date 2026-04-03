use std::time::Duration;

/// Get GCP regional SSL cert
/// Token and project name need to be provided
pub async fn get_regional_ssl(
    token: String,
    project: String,
    region: String,
    gcp_ssl_name: String
) -> Result<(), std::io::Error> {

    let client = awc::Client::default();
    let get_ssl_request = client
        .get(format!("https://compute.googleapis.com/compute/v1/projects/{project}/regions/{region}/sslCertificates/{gcp_ssl_name}"))
        .bearer_auth(&token)
        .insert_header(("Content-Type", "application/json"))
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .expect("Request CREATE global SSL failed");

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
