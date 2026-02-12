// This example shows how to:
// How to create SSL cert in GCP

use devops_armory::cloud::gcp::{
    auth::auth::gcp_get_credentials_token_iam,
    ssl::{create::create_ssl, models::{CreateSSL, DomainsManaged}},
};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {

    // Token and project needs to be provided
    let token = gcp_get_credentials_token_iam().await.unwrap();
    let project = "some project id".to_string();
    
    // GCP SSL data
    let gcp_ssl_body = CreateSSL { 
        name: "SSL cert name".to_string(), 
        managed: DomainsManaged { 
            domains: vec![
                "domain1".to_owned(),
                "domain2".to_string()
            ] 
        }, 
        r#type: "MANAGED".to_string() 
    };

    // Create SSL cert managed
    create_ssl(
        token,
        project, 
        gcp_ssl_body
    ).await.unwrap();

    Ok(())

}
