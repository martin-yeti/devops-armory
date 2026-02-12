// This example shows how to:
// Setup, modify, get, delete and list GKE cluster in GCP

use devops_armory::cloud::gcp::{
    auth::auth::gcp_get_credentials_token_iam,
    gke::cluster::{
        create::create_gke_cluster, 
        delete::delete_gke_cluster, 
        get::get_gke_cluster_info, 
        models::{
            CiDrBlock, 
            GkeCluster, 
            IpAllocationPolicy, 
            MasterAuthorizedNetworksConfig, 
            NetworkConfig, 
            NodeConfig, 
            Nodepools, 
            UpdateCluster, 
            UpdateGkeCluster
        }, 
        update::update_gke_cluster
    }
};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {

    // Token and project name need to be provided
    let token = gcp_get_credentials_token_iam().await.unwrap();
    let project = "some project name".to_string();
    let location = "some cluster location".to_string();

    // Data which needs to be provided for GKE creation
    // Net, subnet, image type, service account among others fields are required
    // Default private nodes are set to true - nodes are using private IP
    // Service IPv4 Cidr Block is IP range used by the cluster
    let gke_cluster = GkeCluster { 
        name: "cluster_name".to_string(), 
        description: "cluster_description".to_string(), 
        loggingService: None, 
        monitoringService: None, 
        network: "path_to_gcp_net".to_string(), 
        subnetwork: "path_to_gcp_subnet".to_string(), 
        nodePools: vec![ Nodepools { 
                name: "some nodepool name".to_string(), 
                config: NodeConfig { 
                    machineType: "vm machine type".to_string(), 
                    diskSizeGb: 20, 
                    // Default value for oauthscope
                    oauthScopes: vec![
                        "https://www.googleapis.com/auth/cloud-platform".to_string()
                    ], 
                    imageType: "image to use for vm nodes".to_string(), 
                    // SA by default is "default", but others can be used
                    serviceAccount: "name of service account used".to_string(), 
                    diskType: "pd or ssd".to_string() 
                }, 
                initialNodeCount: 1
            }
        ], 
        locations: vec![
            "zone1".to_string(),
            "zone2".to_string(),
            "zone3".to_string()
        ], 
        networkConfig: NetworkConfig { 
            defaultEnablePrivateNodes: true , 
            subnetwork: "subnet name".to_string() 
        }, 
        ipAllocationPolicy: IpAllocationPolicy { 
            useIpAliases: true, 
            servicesIpv4CidrBlock: "IP address range".to_string() 
        }
    };

    // Create GKE cluster with above data
    create_gke_cluster(
        token.clone(), 
        project.clone(), 
        location.clone(), 
        gke_cluster
    ).await.unwrap();

    // Get cluster info
    let gke_cluster_name = "some_cluster_name".to_string();

    get_gke_cluster_info(
        token.clone(), 
        project.clone(), 
        location.clone(), 
        gke_cluster_name.clone()
    ).await.unwrap();

    // Delete cluster
    delete_gke_cluster(
        token.clone(), 
        project.clone(), 
        location.clone(), 
        gke_cluster_name.clone()
    ).await.unwrap();

    // Provide GKE update data
    let gke_cluster_update_data = UpdateGkeCluster { 
        projectId: "project-id".to_string(), 
        clusterId: "cluster-id".to_string(), 
        update: UpdateCluster { 
            desiredMasterAuthorizedNetworksConfig: Some(MasterAuthorizedNetworksConfig { 
                enabled: true, 
                cidrBlocks: vec![
                    CiDrBlock { 
                        displayName: "name of whitelisted IP".to_string(), 
                        cidrBlock: "IP address".to_string() 
                    }
                ], 
                gcpPublicCidrsAccessEnabled: false, 
                privateEndpointEnforcementEnabled: true 
            }), 
            desiredLoggingService: None, 
            desiredMonitoringService: None, 
            desiredMonitoringConfig: None 
        } 
    };
    
    // Update GKE cluster with the above
    update_gke_cluster(
        token, 
        project, 
        location, 
        gke_cluster_name, 
        gke_cluster_update_data
    ).await.unwrap();

    Ok(())

}
