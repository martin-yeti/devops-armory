use std::time::Duration;

use super::models::SqlDb;

/// Create SQL database
/// Token, project, SQL instance need to be provided
pub async fn create_sql_db(
    token: String,
    project: String,
    sql_instance_name: String,
    sql_database: SqlDb
) -> Result<(), std::io::Error> {

    let create_sql_db = sql_database;

    let client = awc::Client::default();
    let request = client
        .post(format!("https://sqladmin.googleapis.com/v1/projects/{project}/instances/{sql_instance_name}/databases"))
        .bearer_auth(&token)
        .insert_header(("Content-Type", "application/json"))
        .timeout(Duration::from_secs(30))
        .send_json(&create_sql_db)
        .await
        .expect("Request POST new SQL user could not been sent");

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
