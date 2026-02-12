// This example shows how to:
// Setup VM, list VM, set Metadata project wide and for specific VM

use devops_armory::cloud::gcp::{
    auth::auth::gcp_get_credentials_token_iam, 
    compute::{
        create::create_gcp_vm,
        list::get_gcp_vm_list,
        metadata::{
            global::set_project_meta,
            regional::set_instance_meta
        }, models::{
            ComputeData, ComputeMetadata, CreateVM, VMDisk, VMDiskInitParams, VMNetworkInterface, VMTags
        }
}};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {

    // Token and project ID is required
    // Zone and specs example can be found below
    // Below example set VM without external IP - VMNetworkInterface accessConfigs set to None
    // If you want to use external IP - accessConfigs needs to be populated with VMAccessConfig struct
    let token = gcp_get_credentials_token_iam().await.unwrap_or_default();
    let project_name = "some_project_name".to_string();
    let gcp_vm_zone = "some_zone".to_string();
    let gcp_vm_specs = CreateVM { 
        disk: vec![
            VMDisk { 
                boot: "true".to_string(), 
                initializeParams: VMDiskInitParams {
                    sourceImage: "path_to_source_image_vm".to_string()
                }
            }
        ], 
        machineType: "machine_type_1".to_string(), 
        name: "some_vm_name".to_string(), 
        networkInterfaces: vec![
            VMNetworkInterface { 
                accessConfigs: None, 
                network: "some_net_name".to_string(), 
                subnetwork: "some_subnet_name".to_string() 
            }], 
        tags: VMTags { 
            items: vec![
                "some tags".to_string()
            ] 
        } 
    };

    // Create VM with above data provided
    create_gcp_vm(
        token.clone(), 
        project_name.clone(), 
        gcp_vm_zone, 
        gcp_vm_specs
    ).await.unwrap();

    // Get list of VM
    get_gcp_vm_list(
        token.clone(), 
        project_name.clone()
    ).await.unwrap();

    // Define GCP compute project Metadata
    let gcp_project_metadata = ComputeMetadata { 
        items: vec![
            ComputeData { 
                key: "some_key".to_string(), 
                value: "some_value".to_string() 
            }
        ] 
    };

    // Set project Metadata
    set_project_meta(
        token.to_string(), 
        project_name.to_string(), 
        gcp_project_metadata
    ).await.unwrap();

    // Define VM zone, name and instance metadata
    let gcp_instance_zone = "some_instance_zone".to_string();
    let gcp_vm_name = "some_vm_name".to_string();
    let gcp_instance_metadata = ComputeMetadata { 
        items: vec![
            ComputeData { 
                key: "some_key".to_string(), 
                value: "some_value".to_string() 
            }
        ] 
    };

    // Set instance Metadata
    set_instance_meta(
        token, 
        project_name, 
        gcp_instance_zone, 
        gcp_vm_name, 
        gcp_instance_metadata
    ).await.unwrap();

    Ok(())

}
