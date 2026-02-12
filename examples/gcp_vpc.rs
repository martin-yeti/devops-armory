// This example shows how to:
// Setup, modify, get, list VPC and related in GCP

use devops_armory::cloud::gcp::{
    auth::auth::gcp_get_credentials_token_iam,
    vpc::{
        address::{
            create_global::create_global_ip,
            create_internal::create_internal_address,
            create_regional::create_regional_ip, 
            models::IpAddressRegion
        },
        net::{
            create::create_vpc_network,
            get::get_network_info
        },
        router::{
            create::create_router, 
            models::{
                RouterNats, 
                RouterSubnetConfig
            }
        },
        subnet::{
            create::create_vpc_subnetwork, get::get_subnetwork_info, models::SecondaryIpRanges, private_service_access::set_private_access_for_subnet, update::update_vpc_subnetwork
        },
        vpc_service_conn::create::create_virtual_private_conn
    }
};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {

    // Token and project needs to be provided
    let token = gcp_get_credentials_token_iam().await.unwrap_or_default();
    let project = "gcp project name".to_string();
    let ip_name = "ip_name".to_string();

    // Create global IP in GCP
    create_global_ip(
        token.clone(), 
        project.clone(), 
        ip_name.clone()
    ).await.unwrap();

    // Data required for internal address
    let ip_address = "some_IP_name".to_string();
    let ip_prefix = "0/8/16/24/32_or_any_other".to_string();
    let network_tier = "some network tier".to_string();
    let ip_version = "IPV4_or_IPV6".to_string();
    let ip_address_type = "INTERNAL".to_string();
    let ip_address_purpose = "address_purpose".to_string();
    let network_name = "net_name".to_string();

    // Create internal address
    create_internal_address(
        token.clone(), 
        project.clone(), 
        ip_name, 
        ip_address, 
        ip_prefix, 
        network_tier, 
        ip_version, 
        ip_address_type, 
        ip_address_purpose, 
        network_name
    ).await.unwrap();

    // Data required for regional address
    let region = "some IP region".to_string();
    let ip_regional = IpAddressRegion { 
        name: "IP name".to_string(), 
        networkTier: "network_tier".to_string(), 
        addressType: "address_type".to_string() 
    };

    // Create regional IP
    create_regional_ip(
        token.clone(), 
        project.clone(), 
        region, 
        ip_regional
    ).await.unwrap();

    // Get network info
    let net_name = "some_net_name".to_string();

    get_network_info(
        token.clone(), 
        project.clone(), 
        net_name.clone()
    ).await.unwrap();

    // Data needed for creation of VPC network
    let vpc_description = "some vpc description".to_string();
    let vpc_auto_create_subnet = false;

    // Create VPC net with above data
    create_vpc_network(
        token.clone(), 
        project.clone(), 
        net_name, 
        vpc_description, 
        vpc_auto_create_subnet
    ).await.unwrap();

    // Data required for VPC router creation
    let router_name = "vpc_router_name".to_string();
    // Below uses manual IP allocation for NAT
    let router_nats = vec![ 
        RouterNats { 
            natIpAllocateOption: "MANUAL_ONLY".to_string(), 
            natIps: vec![
                "IP_url_1".to_string(),
                "IP_url_2".to_string()
            ], 
            name: "some_nat_name".to_string(), 
            udpIdleTimeoutSec: 30, 
            subnetworks: vec![ RouterSubnetConfig { 
                sourceIpRangesToNat: vec![
                    "IP_range_to_NAT".to_string()
                ], 
                name: "complete_url_to_your_subnet".to_string() 
            }], 
            r#type: "some_nat_type".to_string(), 
            tcpTimeWaitTimeoutSec: 120, 
            icmpIdleTimeoutSec: 30, 
            tcpTransitoryIdleTimeoutSec: 30, 
            endpointTypes: vec![
                "endpoint_type".to_string()
            ], 
            tcpEstablishedIdleTimeoutSec: 1200, 
            enableEndpointIndependentMapping: true, 
            autoNetworkTier: "net_tier".to_string(), 
            sourceSubnetworkIpRangesToNat: "LIST_OF_SUBNETWORKS".to_string() 
        }
    ];
    let net_name = "some_net_name".to_string();
    let region = "some_router_region".to_string();

    // Create VPC router with above data
    create_router(
        token.clone(), 
        project.clone(), 
        router_name, 
        router_nats, 
        net_name, 
        region.clone()
    ).await.unwrap();

    // Get VPC subnet
    let subnet_name = "some_subnet_name".to_string();

    get_subnetwork_info(
        token.clone(), 
        project.clone(), 
        region.clone(), 
        subnet_name.clone()
    ).await.unwrap();

    // Data needed to be provided for creation VPC subnet
    let subnet_desc = "some_subnet_description".to_string();
    let subnet_network = "full_path_to_net".to_string();
    let subnet_ipcidrrange = "IP_Range".to_string();

    // Create VPC subnet with above data
    create_vpc_subnetwork(
        token.clone(), 
        project.clone(), 
        region.clone(), 
        subnet_name.clone(), 
        subnet_desc, 
        subnet_network, 
        subnet_ipcidrrange
    ).await.unwrap();

    // Set Private access for subnet
    let subnet_private_access = true;
    set_private_access_for_subnet(
        token.clone(), 
        project.clone(), 
        region.clone(), 
        subnet_name.clone(), 
        subnet_private_access
    ).await.unwrap();

    // Update VPC subnet data
    let subnet_sec_ip_ranges = vec![ SecondaryIpRanges { 
        rangeName: "some_ip_name".to_string(), 
        ipCidrRange: "some_ip_range".to_string()
    }];

    // Subnet fingerprint is required
    let subnet_fingerprint = "subnet_fingerprint".to_string();

    // Update VPC subnet with above data
    update_vpc_subnetwork(
        token.clone(), 
        project, 
        subnet_name, 
        region, 
        subnet_sec_ip_ranges, 
        subnet_fingerprint
    ).await.unwrap();
    
    // Data for virtual priv conn
    let net_url = "path_to_network".to_string();
    let net_res_ip_ranges = vec![
        "some_internal_ip_name_1".to_string(),
        "some_internal_ip_name_2".to_string()
    ];

    // Create Virtual Private Connection with above data
    create_virtual_private_conn(
        token, 
        net_url, 
        net_res_ip_ranges
    ).await.unwrap();

    Ok(())

}

