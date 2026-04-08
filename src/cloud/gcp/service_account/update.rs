use std::time::Duration;

use super::models::UpdateServiceAccount;

/// Patch Service Account
/// Need to provide token, project, service account data to successfully send request
pub async fn patch_service_account(
    token: String,
    project: String,
    service_account_name: String,
    service_account_data: UpdateServiceAccount
) -> Result<(), std::io::Error> {

    let client = awc::Client::default();
    let request = client
        .put(format!("https://iam.googleapis.com/v1/projects/{project}/serviceAccounts/{service_account_name}"))
        .bearer_auth(&token)
        .insert_header(("Content-Type", "application/json"))
        .timeout(Duration::from_secs(30))
        .send_json(&service_account_data)
        .await
        .expect("Request: PUT service account failed.");

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
