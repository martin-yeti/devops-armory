use std::time::Duration;

/// Delete DNS record
/// Need to provide project, managed zone and token to successfully send request
pub async fn delete_record_set(
    project: String,
    managed_zone: String,
    token: String,
    dns_name: String,
    dns_type: String,
) -> Result<(), std::io::Error> {

    let client = awc::Client::default();
    let request = client.delete(format!("https://dns.googleapis.com/dns/v1/projects/{project}/managedZones/{managed_zone}/rrsets/{dns_name}/{dns_type}"))
        .bearer_auth(&token)
        .insert_header(("Content-Type", "application/json"))
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .unwrap();

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

