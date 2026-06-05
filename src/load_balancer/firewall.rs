use actix_web::web;
use tokio::process::Command;

/// If path is not empty, then block provided suffixes
pub fn suspicious_path(
    path: web::Data<String>,
) -> bool {
    !path.is_empty()
}

/// IPTables wrapper for blocking IP 
/// which are trying to access suspicious_path()
/// This will require sudo/root access
/// Program needs to be "sudo" if not called as root
/// Then point to script with iptables bash script with 700 permissions
pub async fn block_ip(
    program: &str,
    script: &str,
    ip: &str
) {

    let result = Command::new(program)
        .arg(script)
        .args([ip])
        .output()
        .await;

    match result {
        Ok(o) if o.status.success() => log::warn!("Blocked {ip} via iptables"),
        Ok(o) => log::error!("iptables failed for {ip}: {}", String::from_utf8_lossy(&o.stderr)),
        Err(e) => log::error!("Failed to exec iptables for {ip}: {e}"),
    }

}
