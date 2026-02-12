// This example shows how to:
// Setup, modify, get, delete and list DNS records in GCP

use devops_armory::cloud::gcp::{
    auth::auth::gcp_get_credentials_token_iam,
    dns::{
        create::create_record_set,
        delete::delete_record_set,
        update::update_record_set,
        list::list_records_set,
        get::get_records_set
    }
};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {

    // Token, project name, dns data need to be provided
    let token = gcp_get_credentials_token_iam().await.unwrap_or_default();
    let project_name = "some_project_name".to_string();
    let managed_zone = "some_dns_zone".to_string();
    let dns_name = "some_dns_name".to_string();
    let dns_type = "some_dns_type".to_string();
    let propagation_ttl = 600;
    let dns_rrdatas = vec![
        "some_IP_or_other_data".to_string()
    ];
    let dns_signatureRrdatas = None;
    let dns_kind = "dns#resourceRecordSet".to_string();

    // Create DNS record
    create_record_set(
        project_name.clone(), 
        managed_zone.clone(), 
        token.clone(), 
        dns_name.clone(), 
        dns_type.clone(), 
        propagation_ttl, 
        dns_rrdatas.clone(), 
        dns_signatureRrdatas.clone(), 
        dns_kind.clone()
    ).await.unwrap();

    // Delete DNS record
    delete_record_set(
        project_name.clone(), 
        managed_zone.clone(), 
        token.clone(), 
        dns_name.clone(), 
        dns_type.clone()
    ).await.unwrap();

    // Update DNS record
    update_record_set(
        project_name.clone(), 
        managed_zone.clone(), 
        token.clone(), 
        dns_name.clone(), 
        dns_type.clone(), 
        propagation_ttl, 
        dns_rrdatas, 
        dns_signatureRrdatas, 
        dns_kind
    ).await.unwrap();

    // Get DNS record info
    get_records_set(
        token.clone(), 
        project_name.clone(), 
        managed_zone.clone(), 
        dns_name, 
        dns_type
    ).await.unwrap();

    // List DNS records
    list_records_set(
        token, 
        project_name, 
        managed_zone
    ).await.unwrap();

    Ok(())

}
