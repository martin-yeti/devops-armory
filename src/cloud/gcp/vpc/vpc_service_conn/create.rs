use std::time::Duration;
use super::models::PrivateServiceConnection;

/// Create Virtual Private Connection
/// Token need to be provided
pub async fn create_virtual_private_conn(
    token: String,
    net_url: String,
    net_res_ip_ranges: Vec<String>
) -> Result<(), std::io::Error> {

    let virt_priv_conn: PrivateServiceConnection = PrivateServiceConnection { 
        network: net_url,
        reservedPeeringRanges: net_res_ip_ranges 
    };

    let client = awc::Client::default();
    let request = client.post(format!("https://servicenetworking.googleapis.com/v1/services/servicenetworking.googleapis.com/connections"))
        .bearer_auth(&token)
        .insert_header(("Content-Type", "application/json"))
        .timeout(Duration::from_secs(30))
        .send_json(&virt_priv_conn)
        .await
        .expect("Request create private service connection could not been sent");

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

