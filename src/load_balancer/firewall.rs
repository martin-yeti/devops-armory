use tokio::process::Command;

/// Checks a client IP against the configured allowlist.
/// `None` means no restriction is configured - every IP is allowed.
pub fn is_ip_allowed(allowed_ips: &Option<Vec<String>>, client_ip: &str) -> bool {
    match allowed_ips {
        None => true,
        Some(ips) => ips.iter().any(|ip| ip == client_ip),
    }
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
