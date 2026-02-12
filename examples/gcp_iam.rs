// This example shows how to list and update GCP IAM roles

use devops_armory::cloud::gcp::{
    auth::auth::gcp_get_credentials_token_iam, 
    iam::{
    list::get_iam_gcp_project,
    update::update_iam_gcp_project_policy
}};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {

    // Token and project ID is required
    let token = gcp_get_credentials_token_iam().await.unwrap_or_default();
    let project = "some_project_id".to_string();

    // Get IAM policy info
    get_iam_gcp_project(
        token.clone(), 
        project.clone()
    ).await.unwrap();

    // Set role for user(s)
    let iam_role = "some iam role".to_string();
    let iam_members = vec![
        "Some user 1".to_string(), 
        "Some user 2".to_string()
        ];

    // Update roles for the above
    update_iam_gcp_project_policy(token, project, iam_role, iam_members).await;

    Ok(())

}
