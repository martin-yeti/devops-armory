use std::time::Duration;

use super::models::CreateDNSRecord;

/// Create DNS records
/// Need to provide project, managed zone and token to successfully send request
/// RRdatas contains list of values
pub async fn create_record_set(
    project: String,
    managed_zone: String,
    token: String,
    dns_name: String,
    dns_type: String,
    propagation_ttl: i32,
    dns_rrdatas: Vec<String>,
    dns_signatureRrdatas: Option<Vec<String>>,
    dns_kind: String
) -> Result<(), std::io::Error> {

    let dns_data = CreateDNSRecord {
            name: dns_name,
            r#type: dns_type,
            ttl: propagation_ttl,
            rrdatas: dns_rrdatas,
            signatureRrdatas: None,
            kind: dns_kind
    };

    let client = awc::Client::default();
    let request = client.post(format!("https://dns.googleapis.com/dns/v1/projects/{project}/managedZones/{managed_zone}/rrsets"))
        .bearer_auth(&token)
        .insert_header(("Content-Type", "application/json"))
        .timeout(Duration::from_secs(30))
        .send_json(&dns_data)
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

