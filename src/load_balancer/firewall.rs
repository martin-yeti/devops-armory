use std::net::IpAddr;

use ipnet::IpNet;
use tokio::process::Command;

/// Checks a client IP against the configured allowlist.
/// `None` means no restriction is configured - every IP is allowed.
/// Each entry may be a single address ("127.0.0.1") or a CIDR block
/// ("10.0.0.0/24"); malformed entries are ignored (logged, not matched).
pub fn is_ip_allowed(allowed_ips: &Option<Vec<String>>, client_ip: &str) -> bool {
    let Some(ips) = allowed_ips else {
        return true;
    };

    let Ok(client_addr) = client_ip.parse::<IpAddr>() else {
        log::warn!("Can't parse client IP {client_ip:?}, denying");
        return false;
    };

    ips.iter().any(|entry| {
        if let Ok(net) = entry.parse::<IpNet>() {
            net.contains(&client_addr)
        } else if let Ok(addr) = entry.parse::<IpAddr>() {
            addr == client_addr
        } else {
            log::warn!("Invalid allowed_ips entry, ignoring: {entry:?}");
            false
        }
    })
}

/// For debug purpose only
pub fn suspicious_path(
    forbidden_path: String,
    path: String
) -> bool {
    forbidden_path.starts_with(&path)
}

/// IPTables wrapper for blocking IP 
/// which are trying to access forbidden_path
/// This will require sudo/root access
/// Program needs to be "sudo" if not called as root
/// Then point to script with iptables bash script with 700 permissions
pub async fn block_ip(
    sudo_executor: String,
    script_location: String,
    ip: &str
) {
    println!("{sudo_executor}");
    println!("{script_location}");
    let result = Command::new(sudo_executor)
        .arg(script_location)
        .args([ip])
        .output()
        .await;

    match result {
        Ok(o) if o.status.success() => log::warn!("Blocked {ip} via iptables"),
        Ok(o) => log::error!("iptables failed for {ip}: {}", String::from_utf8_lossy(&o.stderr)),
        Err(e) => log::error!("Failed to exec iptables for {ip}: {e}"),
    }

}
