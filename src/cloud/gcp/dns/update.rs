use super::models::CreateDNSRecord;

/// Update DNS records
/// Need to provide project, managed zone and token to successfully send request
/// RRdatas contains list of values
pub async fn update_record_set(
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
            name: dns_name.clone(),
            r#type: dns_type.clone(),
            ttl: propagation_ttl,
            rrdatas: dns_rrdatas,
            signatureRrdatas: None,
            kind: dns_kind
    };

    let client = awc::Client::default();
    let request = client.patch(format!("https://dns.googleapis.com/dns/v1/projects/{project}/managedZones/{managed_zone}/rrsets/{dns_name}/{dns_type}"))
        .bearer_auth(&token)
        .insert_header(("Content-Type", "application/json"))
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

