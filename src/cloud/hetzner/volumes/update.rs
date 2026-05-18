use std::time::Duration;

use super::models::UpdateHetznerVolume;

/// Update Hetzner volume 
/// Token needs to be provided
pub async fn update_hetzner_volume(
    token: String,
    hetzner_volume_id: i64,
    hetzner_vm_volume: UpdateHetznerVolume
) -> Result<(), std::io::Error> {

    let hetzner_vm_data = hetzner_vm_volume;

    let client = awc::Client::default();
    let request = client
        .put(format!("https://api.hetzner.cloud/v1/volumes/{hetzner_volume_id}"))
        .bearer_auth(&token)
        .insert_header(("Content-Type", "application/json"))
        .timeout(Duration::from_secs(30))
        .send_json(&hetzner_vm_data)
        .await
        .expect("Request PUT volume could not been sent");

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
