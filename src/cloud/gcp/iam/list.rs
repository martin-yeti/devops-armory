use std::time::Duration;

/// Get IAM policy for GCP project
/// Token and project id are required
pub async fn get_iam_gcp_project(
    token: String,
    project: String
) -> Result<(), std::io::Error> {

    let client = awc::Client::default();
    let request = client
        .post(format!("https://cloudresourcemanager.googleapis.com/v3/projects/{project}:getIamPolicy"))
        .bearer_auth(token)
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .expect("Request: GET Project IAM List could not been sent");

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

