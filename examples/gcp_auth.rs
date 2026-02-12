// This example shows how to use different method of authenticating to GCP
// And how TOML file can be used to store configuration

use devops_armory::{cloud::gcp::auth::auth::{
    gcp_get_authentication_method, 
    gcp_get_credentials_token_iam, 
    gcp_get_credentials_token_sa
    }, 
    toml_parser::parser::toml_parser
};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {

    // Wrapper function to choose auth method
    // You can use "IAM" or "Service Account"
    // to authenticate the request
    let auth_method= "IAM".to_string();

    gcp_get_authentication_method(auth_method).await?;

    // Or you can choose directly IAM method
    gcp_get_credentials_token_iam().await?;

    // Or you can choose directly Service Account method
    // Project and credentials needs to be provided
    // Path to credentials can be a raw string - direct path 
    // Or parsed path from TOML file
    let gcp_project = "gcp_project".to_string();
    let gcp_credentials = "path_to_your_sa_credentials.json".to_string();

    gcp_get_credentials_token_sa(
        Some(gcp_project), 
        Some(gcp_credentials)
    ).await?;

    // Example how to use TOML parsing
    // to get token from specific SA file 
    let gcp_credentials_toml = toml_parser(
        "../toml/config.toml".to_string()
    ).unwrap();
    
    // Below parse 1st index of cloud array
    let gcp_toml_cred_file = gcp_credentials_toml.cloud.get(0).unwrap();
    
    // Below parse name after cloud index 
    let y = "gcp".to_string();

    // Below parse credentials file path based on above
    let x = &gcp_toml_cred_file.projects[&y][0].credentials;
        
    gcp_get_credentials_token_sa(
        Some(y), 
        Some(x.to_string())
    ).await?;
    
    Ok(())

}
