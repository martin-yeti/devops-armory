use std::time::Duration;

use super::models::BillingInfo;

/// Add billing info to GCP project
/// Token and billing account are required
pub async fn add_billing_to_gcp_project(
    token: String,
    project: String,
    billing_account: String
) -> Result<(), std::io::Error> {

    let billing_info_body: BillingInfo = BillingInfo { 
        billingAccountName: billing_account
    }; 

    let client = awc::Client::default();
    let request = client.put(format!("https://cloudbilling.googleapis.com/v1/projects/{project}/billingInfo"))
        .bearer_auth(token)
        .insert_header(("Content-Type", "application/json"))
        .timeout(Duration::from_secs(30))
        .send_json(&billing_info_body)
        .await
        .expect("Request: PUT Update billing info could not been sent");

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
