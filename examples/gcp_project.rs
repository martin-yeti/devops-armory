// This example shows how to list all projects in GCP organization
// or create one

use devops_armory::cloud::gcp::{
    auth::auth::gcp_get_credentials_token_iam, 
    project::{
        list::list_gcp_projects,
        create::create_gcp_project
    }
};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    
    // First, we need to get token
    // Second, you will need org id
    let token = gcp_get_credentials_token_iam().await.unwrap_or_default();
    let org_id = "some_organization_id".to_string();

    // Then you can list all projects for entire organization
    list_gcp_projects(
        token.clone(), 
        org_id.clone()
    ).await.unwrap_or_default();

    // You can create project as well and some data will be needed, like
    // id, state, and display name
    let project_id = "some_project_id".to_string();
    let project_state = "ACTIVE".to_string();
    let project_display_name = "Name To Display".to_string();

    // Then you can create the project
    create_gcp_project(
        token.clone(), 
        org_id.clone(), 
        project_id, 
        project_state, 
        project_display_name
    ).await.unwrap_or_default();

    Ok(())

}
