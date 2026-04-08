use std::time::Duration;

/// Delete Service Account
/// Handle with care ! - If you delete SA it might be not possible to restore it
/// Need to provide token, project, service account name to successfully send request
pub async fn delete_service_account(
    token: String,
    project: String,
    service_account_name: String
) -> Result<(), std::io::Error> {

    let client = awc::Client::default();
    let request = client
        .delete(format!("https://iam.googleapis.com/v1/projects/{project}/serviceAccounts/{service_account_name}"))
        .bearer_auth(&token)
        .insert_header(("Content-Type", "application/json"))
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .expect("Request: DELETE service account failed.");

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
