use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Clone)]
pub struct Upstreams {
    pub addrs: Arc<Vec<String>>,
    pub idx: Arc<AtomicUsize>,
}

impl Upstreams {
    pub fn new(addrs: Vec<String>) -> Self {
        Self {
            addrs: Arc::new(addrs),
            idx: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn next(&self) -> String {
        let i = self.idx.fetch_add(1, Ordering::Relaxed);
        let n = self.addrs.len();
        self.addrs[i % n].clone()
    }

}

#[derive(Debug)]
pub struct ForbiddenPath(pub Vec<String>);
pub struct SudoExecutor(pub String);
pub struct ScriptLocation(pub String);

/// IP allowlist. `None` means unrestricted - every client is allowed.
/// `Some(ips)` restricts access to only those addresses.
#[derive(Debug, Clone)]
pub struct AllowedIps(pub Option<Vec<String>>);

/// TLS termination config: paths to a PEM certificate chain and a PEM
/// PKCS#8 private key. Pass `None` to `server()` to serve plain HTTP.
#[derive(Debug, Clone)]
pub struct TlsConfig {
    pub cert_path: String,
    pub key_path: String,
}
