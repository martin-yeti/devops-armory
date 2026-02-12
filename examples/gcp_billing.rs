// This example shows how to add billing account to GCP project

use devops_armory::cloud::gcp::{auth::auth::gcp_get_credentials_token_iam, project::billing::add_billing_to_gcp_project};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {

    // First we need to obtain token
    // And set details like project id and billing account
    let token = gcp_get_credentials_token_iam().await.unwrap_or_default();
    let project = "some_project_id".to_string();
    let billing_account = "some_billing_account".to_string();

    // Then above is send to GCP api
    add_billing_to_gcp_project(
        token, 
        project, 
        billing_account
    ).await.unwrap_or_default();

    Ok(())

}
