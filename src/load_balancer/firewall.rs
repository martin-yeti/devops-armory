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

/// Resolves a raw request path the way the upstream will before routing
/// it: percent-decodes it, drops empty and "." segments, and applies ".."
/// segments. Matching against the raw path instead would let
/// "/build/../admin" pass a "/build" allowlist entry, and "/%77p-admin" or
/// "//wp-admin" slip past a "/wp" block.
/// A trailing slash is kept, so a "/wp/" entry still matches "/wp/".
/// Returns `None` for a path no legitimate client sends: one that climbs
/// above the root, decodes to invalid UTF-8, or contains a backslash or NUL.
pub fn normalize_path(raw_path: &str) -> Option<String> {
    let decoded = urlencoding::decode(raw_path).ok()?;
    if decoded.contains(['\\', '\0']) {
        return None;
    }

    let mut segments: Vec<&str> = Vec::new();
    for segment in decoded.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                segments.pop()?;
            }
            segment => segments.push(segment),
        }
    }

    let mut normalized = format!("/{}", segments.join("/"));
    if !segments.is_empty() && decoded.ends_with('/') {
        normalized.push('/');
    }
    Some(normalized)
}

/// Returns the first forbidden entry that `path` starts with, if any.
/// Matching is a plain prefix match, so "/wp" catches "/wp-admin",
/// "/wp-login.php" and "/wp/..." alike. `path` must already be
/// normalized with [`normalize_path`]. Blank entries are skipped, since
/// as a prefix they would match, and block, every client.
pub fn matching_forbidden_path<'a>(forbidden_paths: &'a [String], path: &str) -> Option<&'a str> {
    forbidden_paths
        .iter()
        .map(|entry| entry.trim())
        .find(|entry| !entry.is_empty() && path.starts_with(entry))
}

/// Checks a request path against the allowed path list.
/// An entry admits itself and everything below it, so "/build" admits
/// "/build", "/build/" and "/build/wasm_object.xyz", but not
/// "/build-secret". "/" admits only the root page: as a prefix it would
/// admit every path and make the allowlist a no-op.
/// `path` must already be normalized with [`normalize_path`].
pub fn is_path_allowed(allowed_paths: &[String], path: &str) -> bool {
    allowed_paths.iter().any(|entry| {
        let prefix = entry.trim().trim_end_matches('/');
        if prefix.is_empty() {
            return path == "/";
        }
        path.strip_prefix(prefix)
            .is_some_and(|rest| rest.is_empty() || rest.starts_with('/'))
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
    log::warn!("Using sudo executor: {sudo_executor}");
    log::warn!("Executing blocking script: {script_location}");
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

#[cfg(test)]
mod tests {

    use super::{is_path_allowed, matching_forbidden_path, normalize_path};

    fn paths(entries: &[&str]) -> Vec<String> {
        entries.iter().map(|e| e.to_string()).collect()
    }

    #[test]
    fn normalize_path_resolves_like_upstream() {
        assert_eq!(normalize_path("/").as_deref(), Some("/"));
        assert_eq!(normalize_path("/build/app.wasm").as_deref(), Some("/build/app.wasm"));
        assert_eq!(normalize_path("//wp-admin").as_deref(), Some("/wp-admin"));
        assert_eq!(normalize_path("/./wp-admin").as_deref(), Some("/wp-admin"));
        assert_eq!(normalize_path("/%77p-admin").as_deref(), Some("/wp-admin"));
        assert_eq!(normalize_path("/build/../admin").as_deref(), Some("/admin"));
        assert_eq!(normalize_path("/build/..%2Fadmin").as_deref(), Some("/admin"));
        assert_eq!(normalize_path("/build/%2e%2e/admin").as_deref(), Some("/admin"));
        assert_eq!(normalize_path("/wp/").as_deref(), Some("/wp/"));
    }

    #[test]
    fn normalize_path_rejects_malformed() {
        assert_eq!(normalize_path("/../etc/passwd"), None);
        assert_eq!(normalize_path("/build/../../etc/passwd"), None);
        assert_eq!(normalize_path("/build\\..\\admin"), None);
        assert_eq!(normalize_path("/build%5c..%5cadmin"), None);
        assert_eq!(normalize_path("/build%00.wasm"), None);
        assert_eq!(normalize_path("/build/%ff"), None);
    }

    #[test]
    fn forbidden_path_is_a_prefix_match() {
        let forbidden = paths(&["/wp", " /.env ", ""]);
        assert_eq!(matching_forbidden_path(&forbidden, "/wp"), Some("/wp"));
        assert_eq!(matching_forbidden_path(&forbidden, "/wp-admin"), Some("/wp"));
        assert_eq!(matching_forbidden_path(&forbidden, "/wp-login.php"), Some("/wp"));
        assert_eq!(matching_forbidden_path(&forbidden, "/wp/xmlrpc.php"), Some("/wp"));
        assert_eq!(matching_forbidden_path(&forbidden, "/.env.local"), Some("/.env"));
        assert_eq!(matching_forbidden_path(&forbidden, "/build/wp"), None);
        assert_eq!(matching_forbidden_path(&forbidden, "/"), None);
    }

    #[test]
    fn allowed_path_admits_its_subtree() {
        let allowed = paths(&["/build", "/static/ "]);
        assert!(is_path_allowed(&allowed, "/build"));
        assert!(is_path_allowed(&allowed, "/build/"));
        assert!(is_path_allowed(&allowed, "/build/wasm_object.xyz"));
        assert!(is_path_allowed(&allowed, "/build/nested/app.wasm"));
        assert!(is_path_allowed(&allowed, "/static/app.css"));
        assert!(!is_path_allowed(&allowed, "/build-secret"));
        assert!(!is_path_allowed(&allowed, "/buildx/app.wasm"));
        assert!(!is_path_allowed(&allowed, "/"));
        assert!(!is_path_allowed(&allowed, "/admin"));
    }

    #[test]
    fn root_allowed_path_admits_only_root() {
        let allowed = paths(&["/"]);
        assert!(is_path_allowed(&allowed, "/"));
        assert!(!is_path_allowed(&allowed, "/admin"));
    }

}
