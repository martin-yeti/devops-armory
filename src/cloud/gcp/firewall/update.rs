use std::time::Duration;

use super::models::FirewallRule;

/// Update Firewall Rule
/// Need to provide project, token to successfully send request
pub async fn update_firewall_rule(
    project: String,
    token: String,
    firewall_rule_name: String,
    firewall_rule: FirewallRule
) -> Result<(), std::io::Error> {

    let firewall_data = firewall_rule;

    let client = awc::Client::default();
    let request = client
        .put(format!("https://compute.googleapis.com/compute/v1/projects/{project}/global/firewalls/{firewall_rule_name}"))
        .bearer_auth(&token)
        .insert_header(("Content-Type", "application/json"))
        .timeout(Duration::from_secs(30))
        .send_json(&firewall_data)
        .await
        .expect("Request UPDATE Firewall rule could not be send");

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

