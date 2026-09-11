use devops_armory::load_balancer::{
    server::server,
    models::TlsConfig
};

// This example shows how to create simple load balancer
// Supports HTTPS upstreams
// Provide blocked paths, port, sudo exec and script for blocking IP in iptables
async fn create_simple_load_balancer() -> Result<(), std::io::Error> {

    rustls::crypto::ring::default_provider().install_default().expect("Failed to install rustls crypto provider");

    let blocked_paths = vec![
        "/.sd".to_string(),
        "/.dsa".to_string()
    ];

    let upstream_list = vec![ "https://example.org".to_string()];

    // Only these addresses may reach the load balancer; entries can be single
    // IPs or CIDR blocks; pass None to allow everyone
    let allowed_ips = Some(vec![
        "127.0.0.1".to_string(),
        "10.0.0.0/24".to_string()
    ]);

    // TLS termination is optional; pass None to serve plain HTTP instead
    let tls_config = Some(TlsConfig {
        cert_path: "path_to_your_cert.pem".to_string(),
        key_path: "path_to_your_key.pem".to_string(),
    });

    server(
        "debug".to_string(),
        upstream_list.clone(),
        8080,
        blocked_paths,
        "sudo".to_string(),
        "script_for_blocking_ip".to_string(),
        allowed_ips,
        tls_config
    ).await.unwrap();

    Ok(())

}

#[actix_web::main]
async fn main() {
    create_simple_load_balancer().await.unwrap();
}
